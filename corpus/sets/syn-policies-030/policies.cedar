// Policy 0: Any User can create a list and see what lists they own
permit (
  principal,
  action in [Action::"CreateList", Action::"GetLists"],
  resource == Application::"TinyTodo"
);

// Policy 1: A User can perform any action on a List they own
permit (
  principal,
  action,
  resource is List
)
when { resource.owner == principal };

// Policy 2: A User can see a List if they are either a reader or editor
permit (
  principal,
  action == Action::"GetList",
  resource
)
when { principal in resource.readers || principal in resource.editors };

// Policy 3: A User can update a List and its tasks if they are an editor
permit (
  principal,
  action in
    [Action::"UpdateList",
     Action::"CreateTask",
     Action::"UpdateTask",
     Action::"DeleteTask"],
  resource
)
when { principal in resource.editors };

// Policy 4: Admins can perform any action on any resource
// @id("admin-omnipotence")
// permit (
//    principal in Team::"admin",
//    action,
//    resource in Application::"TinyTodo"
// );
//
// Policy 5: Interns may not create new task lists
// forbid (
//     principal in Team::"interns",
//     action == Action::"CreateList",
//     resource == Application::"TinyTodo"
// );
//
// Policy 6: No access if not high rank and at location DEF,
// or at resource's owner's location
// forbid(
//     principal,
//     action,
//     resource is List
// ) unless {
//     principal.joblevel > 6 && principal.location like "DEF*" ||
//     principal.location == resource.owner.location
// };
@id("extra0")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-0" && principal.joblevel > 0 };

@id("extra1")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-1" && principal.joblevel > 1 };

@id("extra2")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-2" && principal.joblevel > 2 };

@id("extra3")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-3" && principal.joblevel > 3 };

@id("extra4")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-4" && principal.joblevel > 4 };

@id("extra5")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-5" && principal.joblevel > 5 };

@id("extra6")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-6" && principal.joblevel > 6 };

@id("extra7")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-7" && principal.joblevel > 7 };

@id("extra8")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-8" && principal.joblevel > 8 };

@id("extra9")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-9" && principal.joblevel > 0 };

@id("extra10")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-10" && principal.joblevel > 1 };

@id("extra11")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-11" && principal.joblevel > 2 };

@id("extra12")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-12" && principal.joblevel > 3 };

@id("extra13")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-13" && principal.joblevel > 4 };

@id("extra14")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-14" && principal.joblevel > 5 };

@id("extra15")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-15" && principal.joblevel > 6 };

@id("extra16")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-16" && principal.joblevel > 7 };

@id("extra17")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-17" && principal.joblevel > 8 };

@id("extra18")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-18" && principal.joblevel > 0 };

@id("extra19")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-19" && principal.joblevel > 1 };

@id("extra20")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-20" && principal.joblevel > 2 };

@id("extra21")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-21" && principal.joblevel > 3 };

@id("extra22")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-22" && principal.joblevel > 4 };

@id("extra23")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-23" && principal.joblevel > 5 };

@id("extra24")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-24" && principal.joblevel > 6 };

@id("extra25")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-25" && principal.joblevel > 7 };

@id("extra26")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-26" && principal.joblevel > 8 };

@id("extra27")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-27" && principal.joblevel > 0 };

@id("extra28")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-28" && principal.joblevel > 1 };

@id("extra29")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-29" && principal.joblevel > 2 };
