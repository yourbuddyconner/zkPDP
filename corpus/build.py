#!/usr/bin/env python3
"""Assemble corpus/raw from a checkout of cedar-policy/cedar-examples and from seeded generators.

Usage: python3 corpus/build.py <path to cedar-examples checkout>

Output is deterministic. `corpus-tool` then canonicalizes raw sets, runs the
native Cedar evaluator as the oracle, and writes the manifest.
"""
import json, os, random, re, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
RAW = os.path.join(HERE, "raw")
UID = re.compile(r'^(.*)::"((?:[^"\\]|\\.)*)"$')


def split_uid(s):
    m = UID.match(s.strip())
    if not m:
        raise ValueError(f"not an entity uid: {s}")
    return m.group(1), m.group(2).replace('\\"', '"').replace("\\\\", "\\")


def authzen(principal, action, resource, context):
    pt, pid = split_uid(principal)
    at, aid = split_uid(action)
    rt, rid = split_uid(resource)
    name = aid if at == "Action" else action
    return {
        "subject": {"type": pt, "id": pid},
        "action": {"name": name},
        "resource": {"type": rt, "id": rid},
        "context": context,
    }


def write_set(name, policies, schema, entities, requests, note):
    d = os.path.join(RAW, name)
    os.makedirs(d, exist_ok=True)
    open(os.path.join(d, "policies.cedar"), "w").write(policies)
    open(os.path.join(d, "schema.cedarschema"), "w").write(schema)
    json.dump(entities, open(os.path.join(d, "entities.json"), "w"), indent=1, sort_keys=True)
    json.dump(requests, open(os.path.join(d, "requests.json"), "w"), indent=1, sort_keys=True)
    open(os.path.join(d, "NOTE"), "w").write(note + "\n")


# ---------- upstream use cases ----------

def use_cases(examples, commit):
    base = os.path.join(examples, "cedar-example-use-cases")
    for rel in ["github_example", "document_cloud", "streaming_service", "tags_n_roles",
                "hotel_chains/static", "sales_orgs/static"]:
        d = os.path.join(base, rel)
        reqs = []
        for kind in ["ALLOW", "DENY"]:
            qd = os.path.join(d, kind)
            for f in sorted(os.listdir(qd)):
                q = json.load(open(os.path.join(qd, f)))
                reqs.append({
                    "name": f"{kind.lower()}/{f[:-5]}",
                    "upstream_expectation": kind == "ALLOW",
                    "request": authzen(q["principal"], q["action"], q["resource"], q.get("context", {})),
                })
        write_set(
            "usecase-" + rel.replace("/", "-"),
            open(os.path.join(d, "policies.cedar")).read(),
            open(os.path.join(d, "policies.cedarschema")).read(),
            json.load(open(os.path.join(d, "entities.json"))),
            reqs,
            f"From cedar-policy/cedar-examples at {commit}, cedar-example-use-cases/{rel}. Apache-2.0. "
            "Queries converted to AuthZEN requests.",
        )


# ---------- TinyTodo and the synthetic series ----------

def ent(t, i, attrs=None, parents=None):
    return {"uid": {"type": t, "id": i}, "attrs": attrs or {}, "parents": parents or []}


def ref(t, i):
    return {"__entity": {"type": t, "id": i}}


def par(t, i):
    return {"type": t, "id": i}


def tinytodo_store(seed, users, lists, depth=1, strlen=5):
    """Users, three org teams, and per list a readers chain of `depth` teams and an editors team."""
    r = random.Random(seed)
    app = par("Application", "TinyTodo")
    es = [ent("Application", "TinyTodo")]
    for t in ["temp", "admin", "interns"]:
        es.append(ent("Team", t, parents=[app]))
    uparents = {f"u{i}": [app, par("Team", r.choice(["temp", "admin", "interns"]))] for i in range(users)}
    meta = []
    for l in range(lists):
        owner = f"u{r.randrange(users)}"
        # readers chain: readers-l-0 is the team the list names; a reader is a member of the deepest team.
        for k in range(depth):
            parents = [app] if k == 0 else [par("Team", f"readers-{l}-{k-1}")]
            es.append(ent("Team", f"readers-{l}-{k}", parents=parents))
        es.append(ent("Team", f"editors-{l}", parents=[app]))
        others = [u for u in uparents if u != owner]
        reader = r.choice(others)
        editor = r.choice([u for u in others if u != reader] or others)
        uparents[reader].append(par("Team", f"readers-{l}-{depth-1}"))
        uparents[editor].append(par("Team", f"editors-{l}"))
        tasks = [{"id": t, "name": f"task {t}", "state": r.choice(["open", "done"])} for t in range(r.randrange(4))]
        es.append(ent("List", f"l{l}", {
            "owner": ref("User", owner), "name": f"name-{l}",
            "readers": ref("Team", f"readers-{l}-0"), "editors": ref("Team", f"editors-{l}"),
            "tasks": tasks,
        }, [app]))
        meta.append((l, owner, reader, editor))
    for u, ps in uparents.items():
        loc = "".join(r.choice("ABCDEFXYZ") for _ in range(strlen))
        es.append(ent("User", u, {"joblevel": r.randrange(1, 10), "location": loc}, ps))
    return es, meta


def tt_req(name, user, action, rtype, rid, nonce):
    return {"name": name, "request": {
        "subject": {"type": "User", "id": user}, "action": {"name": action},
        "resource": {"type": rtype, "id": rid}, "context": {"nonce": nonce}}}


def tinytodo_requests(meta, users, count):
    reqs = []
    for l, owner, reader, editor in meta[:count]:
        stranger = next(f"u{i}" for i in range(users) if f"u{i}" not in (owner, reader, editor))
        reqs += [
            tt_req(f"l{l}-reader-gets", reader, "GetList", "List", f"l{l}", f"n-{l}-0"),
            tt_req(f"l{l}-stranger-gets", stranger, "GetList", "List", f"l{l}", f"n-{l}-1"),
            tt_req(f"l{l}-editor-updates", editor, "UpdateList", "List", f"l{l}", f"n-{l}-2"),
            tt_req(f"l{l}-reader-updates", reader, "UpdateList", "List", f"l{l}", f"n-{l}-3"),
            tt_req(f"l{l}-owner-deletes", owner, "DeleteList", "List", f"l{l}", f"n-{l}-4"),
            tt_req(f"l{l}-editor-deletes", editor, "DeleteList", "List", f"l{l}", f"n-{l}-5"),
        ]
    reqs.append(tt_req("anyone-creates-list", "u0", "CreateList", "Application", "TinyTodo", "n-app-0"))
    return reqs


def tinytodo(examples, commit):
    tt = os.path.join(examples, "tinytodo")
    policies = open(os.path.join(tt, "policies.cedar")).read()
    schema = open(os.path.join(tt, "tinytodo.cedarschema")).read()
    es, meta = tinytodo_store(seed=1, users=12, lists=9)
    write_set("tinytodo", policies, schema, es, tinytodo_requests(meta, 12, 3),
              f"Policies and schema from cedar-policy/cedar-examples at {commit}, tinytodo. Apache-2.0. "
              f"Entity store generated by corpus/build.py, seed 1: {len(es)} entities. Requests generated.")

    extra = lambda k: (f'\n@id("extra{k}")\npermit (principal, action == Action::"GetList", resource is List)\n'
                       f'when {{ resource.name == "nomatch-{k}" && principal.joblevel > {k % 9} }};\n')
    like = ('\n@id("location")\nforbid (principal, action, resource is List)\n'
            'when { principal.location like "QQQ*" };\n')

    def point(name, note, users=12, lists=9, depth=1, strlen=5, npol=0, with_like=False):
        es, meta = tinytodo_store(seed=2, users=users, lists=lists, depth=depth, strlen=strlen)
        p = policies + "".join(extra(k) for k in range(npol)) + (like if with_like else "")
        write_set(name, p, schema, es, tinytodo_requests(meta, users, 1)[:2],
                  f"Synthetic series, TinyTodo schema, seed 2. {note} {len(es)} entities.")

    # Policy count: the 4 upstream policies plus N that never match.
    for n in [1, 10, 30, 100]:
        point(f"syn-policies-{n:03d}", f"{4 + n} policies.", npol=n)
    # Entity count.
    for users, lists, tag in [(4, 1, "00010"), (30, 22, "00100"), (300, 232, "01000"), (3000, 2332, "10000")]:
        point(f"syn-entities-{tag}", "Entity count series.", users=users, lists=lists)
    # Hierarchy depth of the readers chain.
    for d in [1, 2, 4, 8]:
        point(f"syn-depth-{d}", f"Readers chain depth {d}.", depth=d)
    # String length, with a `like` forbid over the string.
    for s in [8, 32, 128, 256]:
        point(f"syn-strlen-{s:03d}", f"Location length {s} with a like policy.", strlen=s, with_like=True)


if __name__ == "__main__":
    examples = sys.argv[1]
    commit = subprocess.check_output(["git", "-C", examples, "rev-parse", "HEAD"], text=True).strip()
    shutil.rmtree(RAW, ignore_errors=True)
    use_cases(examples, commit)
    tinytodo(examples, commit)
    print("raw sets:", len(os.listdir(RAW)))
