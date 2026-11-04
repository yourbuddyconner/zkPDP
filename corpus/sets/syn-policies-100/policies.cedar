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

@id("extra30")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-30" && principal.joblevel > 3 };

@id("extra31")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-31" && principal.joblevel > 4 };

@id("extra32")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-32" && principal.joblevel > 5 };

@id("extra33")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-33" && principal.joblevel > 6 };

@id("extra34")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-34" && principal.joblevel > 7 };

@id("extra35")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-35" && principal.joblevel > 8 };

@id("extra36")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-36" && principal.joblevel > 0 };

@id("extra37")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-37" && principal.joblevel > 1 };

@id("extra38")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-38" && principal.joblevel > 2 };

@id("extra39")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-39" && principal.joblevel > 3 };

@id("extra40")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-40" && principal.joblevel > 4 };

@id("extra41")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-41" && principal.joblevel > 5 };

@id("extra42")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-42" && principal.joblevel > 6 };

@id("extra43")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-43" && principal.joblevel > 7 };

@id("extra44")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-44" && principal.joblevel > 8 };

@id("extra45")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-45" && principal.joblevel > 0 };

@id("extra46")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-46" && principal.joblevel > 1 };

@id("extra47")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-47" && principal.joblevel > 2 };

@id("extra48")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-48" && principal.joblevel > 3 };

@id("extra49")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-49" && principal.joblevel > 4 };

@id("extra50")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-50" && principal.joblevel > 5 };

@id("extra51")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-51" && principal.joblevel > 6 };

@id("extra52")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-52" && principal.joblevel > 7 };

@id("extra53")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-53" && principal.joblevel > 8 };

@id("extra54")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-54" && principal.joblevel > 0 };

@id("extra55")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-55" && principal.joblevel > 1 };

@id("extra56")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-56" && principal.joblevel > 2 };

@id("extra57")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-57" && principal.joblevel > 3 };

@id("extra58")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-58" && principal.joblevel > 4 };

@id("extra59")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-59" && principal.joblevel > 5 };

@id("extra60")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-60" && principal.joblevel > 6 };

@id("extra61")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-61" && principal.joblevel > 7 };

@id("extra62")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-62" && principal.joblevel > 8 };

@id("extra63")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-63" && principal.joblevel > 0 };

@id("extra64")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-64" && principal.joblevel > 1 };

@id("extra65")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-65" && principal.joblevel > 2 };

@id("extra66")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-66" && principal.joblevel > 3 };

@id("extra67")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-67" && principal.joblevel > 4 };

@id("extra68")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-68" && principal.joblevel > 5 };

@id("extra69")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-69" && principal.joblevel > 6 };

@id("extra70")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-70" && principal.joblevel > 7 };

@id("extra71")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-71" && principal.joblevel > 8 };

@id("extra72")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-72" && principal.joblevel > 0 };

@id("extra73")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-73" && principal.joblevel > 1 };

@id("extra74")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-74" && principal.joblevel > 2 };

@id("extra75")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-75" && principal.joblevel > 3 };

@id("extra76")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-76" && principal.joblevel > 4 };

@id("extra77")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-77" && principal.joblevel > 5 };

@id("extra78")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-78" && principal.joblevel > 6 };

@id("extra79")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-79" && principal.joblevel > 7 };

@id("extra80")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-80" && principal.joblevel > 8 };

@id("extra81")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-81" && principal.joblevel > 0 };

@id("extra82")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-82" && principal.joblevel > 1 };

@id("extra83")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-83" && principal.joblevel > 2 };

@id("extra84")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-84" && principal.joblevel > 3 };

@id("extra85")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-85" && principal.joblevel > 4 };

@id("extra86")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-86" && principal.joblevel > 5 };

@id("extra87")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-87" && principal.joblevel > 6 };

@id("extra88")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-88" && principal.joblevel > 7 };

@id("extra89")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-89" && principal.joblevel > 8 };

@id("extra90")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-90" && principal.joblevel > 0 };

@id("extra91")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-91" && principal.joblevel > 1 };

@id("extra92")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-92" && principal.joblevel > 2 };

@id("extra93")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-93" && principal.joblevel > 3 };

@id("extra94")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-94" && principal.joblevel > 4 };

@id("extra95")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-95" && principal.joblevel > 5 };

@id("extra96")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-96" && principal.joblevel > 6 };

@id("extra97")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-97" && principal.joblevel > 7 };

@id("extra98")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-98" && principal.joblevel > 8 };

@id("extra99")
permit (
  principal,
  action == Action::"GetList",
  resource is List
)
when { resource.name == "nomatch-99" && principal.joblevel > 0 };
