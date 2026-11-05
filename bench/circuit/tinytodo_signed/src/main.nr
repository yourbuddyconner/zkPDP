// The TinyTodo policy set with a signed store, written by hand. Experiment E3.
// As bench/circuit/tinytodo, except the entity root is private and an attribute
// authority's ECDSA P-256 signature over it is checked inside. The verifier pins
// the authority's key and the epoch in place of the root. Decision 0025.
//
// What it proves: for the request whose canonical bytes hash to
// `request_hash`, over the entity store committed by `entities_root`, the
// four TinyTodo policies give `decision`.
//
// Public inputs are the journal of docs/protocol.md, in its order. The
// entity commitment is `merkle-sha256-record-v0`, defined in
// bench/envelope/src/records.rs.
mod consts;

use consts::{POLICY_HASH, SCHEMA_HASH};
use sha256::{digest, sha256_var};

global MAX_TYPE: u32 = 16;
global MAX_ID: u32 = 32;
global MAX_ACTION: u32 = 16;
global MAX_NONCE: u32 = 32;
global MAX_UID: u32 = 52; // type, `::"`, id, `"`
global MAX_REQ: u32 = 256;
global MAX_ATTRS: u32 = 8;
global MAX_ANC: u32 = 16;
global MAX_DEPTH: u32 = 12;

// List attributes in name order: editors, name, owner, readers, tasks.
global LIST_EDITORS: u32 = 0;
global LIST_OWNER: u32 = 2;
global LIST_READERS: u32 = 3;

// One opened leaf of the entity commitment.
struct Entity {
    present: u8, // bit i set when attribute i is present
    attrs: [[u8; 32]; MAX_ATTRS],
    ancestors: [[u8; 32]; MAX_ANC],
    path: [[u8; 32]; MAX_DEPTH],
    sibling_is_left: [bool; MAX_DEPTH],
    path_len: u32,
}

fn eq_str<let N: u32, let M: u32>(s: BoundedVec<u8, N>, lit: str<M>) -> bool {
    let b = lit.as_bytes();
    let mut same = s.len() == M;
    for i in 0..M {
        same = same & (s.get_unchecked(i) == b[i]);
    }
    same
}

// The bytes must be ones that JSON and Cedar write without escaping, so the
// canonical request and the UID text can be rebuilt by concatenation.
fn assert_plain<let N: u32>(s: BoundedVec<u8, N>) {
    assert(s.len() <= N);
    for i in 0..N {
        let c = s.get_unchecked(i);
        let ok = (c >= 0x20) & (c < 0x7f) & (c != 0x22) & (c != 0x5c);
        assert(ok | (i >= s.len()));
        // Padding is zero, so one string has one encoding. E9.
        assert((i < s.len()) | (c == 0));
    }
}

fn push_str<let N: u32, let M: u32>(buf: &mut BoundedVec<u8, N>, lit: str<M>) {
    buf.extend_from_array(lit.as_bytes());
}

// SHA-256 of `Type::"id"`.
fn uid_hash(ty: BoundedVec<u8, MAX_TYPE>, id: BoundedVec<u8, MAX_ID>) -> [u8; 32] {
    let mut buf: BoundedVec<u8, MAX_UID> = BoundedVec::new();
    buf.extend_from_bounded_vec(ty);
    push_str(&mut buf, "::\"");
    buf.extend_from_bounded_vec(id);
    push_str(&mut buf, "\"");
    sha256_var(buf.storage(), buf.len())
}

// SHA-256 of the canonical (JCS) AuthZEN request.
fn request_digest(
    subject_type: BoundedVec<u8, MAX_TYPE>,
    subject_id: BoundedVec<u8, MAX_ID>,
    action: BoundedVec<u8, MAX_ACTION>,
    resource_type: BoundedVec<u8, MAX_TYPE>,
    resource_id: BoundedVec<u8, MAX_ID>,
    nonce: BoundedVec<u8, MAX_NONCE>,
) -> [u8; 32] {
    let mut b: BoundedVec<u8, MAX_REQ> = BoundedVec::new();
    push_str(&mut b, "{\"action\":{\"name\":\"");
    b.extend_from_bounded_vec(action);
    push_str(&mut b, "\"},\"context\":{\"nonce\":\"");
    b.extend_from_bounded_vec(nonce);
    push_str(&mut b, "\"},\"resource\":{\"id\":\"");
    b.extend_from_bounded_vec(resource_id);
    push_str(&mut b, "\",\"type\":\"");
    b.extend_from_bounded_vec(resource_type);
    push_str(&mut b, "\"},\"subject\":{\"id\":\"");
    b.extend_from_bounded_vec(subject_id);
    push_str(&mut b, "\",\"type\":\"");
    b.extend_from_bounded_vec(subject_type);
    push_str(&mut b, "\"}}");
    sha256_var(b.storage(), b.len())
}

fn leaf_hash(uid_h: [u8; 32], e: Entity) -> [u8; 32] {
    let mut a: [u8; 1 + 32 * MAX_ATTRS] = [0; 1 + 32 * MAX_ATTRS];
    a[0] = e.present;
    for i in 0..MAX_ATTRS {
        for j in 0..32 {
            a[1 + 32 * i + j] = e.attrs[i][j];
        }
    }
    let mut c: [u8; 32 * MAX_ANC] = [0; 32 * MAX_ANC];
    for i in 0..MAX_ANC {
        for j in 0..32 {
            c[32 * i + j] = e.ancestors[i][j];
        }
    }
    let ah = digest(a);
    let ch = digest(c);
    let mut l: [u8; 96] = [0; 96];
    for j in 0..32 {
        l[j] = uid_h[j];
        l[32 + j] = ah[j];
        l[64 + j] = ch[j];
    }
    digest(l)
}

fn root_from(leaf: [u8; 32], e: Entity) -> [u8; 32] {
    assert(e.path_len <= MAX_DEPTH);
    let mut h = leaf;
    for i in 0..MAX_DEPTH {
        let mut node: [u8; 65] = [0; 65];
        node[0] = 1;
        for j in 0..32 {
            node[1 + j] = if e.sibling_is_left[i] { e.path[i][j] } else { h[j] };
            node[33 + j] = if e.sibling_is_left[i] { h[j] } else { e.path[i][j] };
        }
        let up = digest(node);
        if i < e.path_len {
            h = up;
        }
    }
    h
}

fn is_in(ancestors: [[u8; 32]; MAX_ANC], group: [u8; 32]) -> bool {
    let mut found = false;
    for i in 0..MAX_ANC {
        found = found | (ancestors[i] == group);
    }
    found
}

fn main(
    // The journal.
    request_hash: pub [u8; 32],
    policy_hash: pub [u8; 32],
    schema_hash: pub [u8; 32],
    authority_x: pub [u8; 32],
    authority_y: pub [u8; 32],
    epoch: pub u64,
    nonce: pub BoundedVec<u8, MAX_NONCE>,
    decision: pub bool,
    // The request, private. The request hash binds it.
    subject_type: BoundedVec<u8, MAX_TYPE>,
    subject_id: BoundedVec<u8, MAX_ID>,
    action: BoundedVec<u8, MAX_ACTION>,
    resource_type: BoundedVec<u8, MAX_TYPE>,
    resource_id: BoundedVec<u8, MAX_ID>,
    // The two entities the policies read, private. The root binds them.
    principal: Entity,
    resource: Entity,
    // The store, private, and the authority's signature over it.
    entities_root: [u8; 32],
    signature: [u8; 64],
) {
    // The authority signed this root for this epoch.
    let mut signed: [u8; 54] = [0; 54];
    let tag = "zk-pdp-root-v0".as_bytes();
    for i in 0..14 {
        signed[i] = tag[i];
    }
    for i in 0..32 {
        signed[14 + i] = entities_root[i];
    }
    let e: [u8; 8] = (epoch as Field).to_be_bytes();
    for i in 0..8 {
        signed[46 + i] = e[i];
    }
    assert(std::ecdsa_secp256r1::verify_signature(authority_x, authority_y, signature, digest(signed)));

    // This circuit is one policy set under one schema.
    assert(policy_hash == POLICY_HASH);
    assert(schema_hash == SCHEMA_HASH);

    // The request is the one the verifier hashed.
    assert_plain(subject_type);
    assert_plain(subject_id);
    assert_plain(action);
    assert_plain(resource_type);
    assert_plain(resource_id);
    assert_plain(nonce);
    assert(
        request_digest(subject_type, subject_id, action, resource_type, resource_id, nonce)
            == request_hash,
    );

    // The entities are the ones in the committed store.
    let p_uid = uid_hash(subject_type, subject_id);
    let r_uid = uid_hash(resource_type, resource_id);
    assert(root_from(leaf_hash(p_uid, principal), principal) == entities_root);
    assert(root_from(leaf_hash(r_uid, resource), resource) == entities_root);

    // The request fits the schema: a User acts, on a List or on the application.
    let is_list = eq_str(resource_type, "List");
    let is_app = eq_str(resource_type, "Application") & eq_str(resource_id, "TinyTodo");
    let get_list = eq_str(action, "GetList");
    let edits = eq_str(action, "UpdateList")
        | eq_str(action, "CreateTask")
        | eq_str(action, "UpdateTask")
        | eq_str(action, "DeleteTask");
    let list_action = get_list | edits | eq_str(action, "DeleteList") | eq_str(action, "EditShare");
    let app_action = eq_str(action, "CreateList") | eq_str(action, "GetLists");
    assert(eq_str(subject_type, "User"));
    assert((list_action & is_list) | (app_action & is_app));
    assert(!is_list | (resource.present == 31)); // all five List attributes

    let in_readers = is_in(principal.ancestors, resource.attrs[LIST_READERS]);
    let in_editors = is_in(principal.ancestors, resource.attrs[LIST_EDITORS]);

    // Policy 0: any User can create a list and see the lists they own.
    let p0 = app_action & is_app;
    // Policy 1: a User can do anything to a List they own.
    let p1 = is_list & (resource.attrs[LIST_OWNER] == p_uid);
    // Policy 2: a User can see a List if they are a reader or an editor.
    let p2 = get_list & is_list & (in_readers | in_editors);
    // Policy 3: a User can update a List and its tasks if they are an editor.
    let p3 = edits & is_list & in_editors;

    // No forbid policies. Permit if any permit applies.
    assert(decision == (p0 | p1 | p2 | p3));
}
