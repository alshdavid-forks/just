use super::*;

#[test]
fn list_displays_recipes_in_submodules() {
  Test::new()
    .write("foo.just", "bar:\n @echo FOO")
    .justfile(
      "
        mod foo
      ",
    )
    .test_round_trip(false)
    .arg("--unstable")
    .arg("--list")
    .stdout(
      "
      Available recipes:
          foo:
              bar
    ",
    )
    .run();
}

#[test]
fn modules_are_space_separated_in_output() {
  Test::new()
    .write("foo.just", "foo:")
    .write("bar.just", "bar:")
    .justfile(
      "
        mod foo

        mod bar
      ",
    )
    .test_round_trip(false)
    .args(["--unstable", "--list"])
    .stdout(
      "
      Available recipes:
          bar:
              bar

          foo:
              foo
    ",
    )
    .run();
}

#[test]
fn modules_unsorted() {
  Test::new()
    .write("foo.just", "foo:")
    .write("bar.just", "bar:")
    .justfile(
      "
        mod foo

        mod bar
      ",
    )
    .test_round_trip(false)
    .args(["--unstable", "--list", "--unsorted"])
    .stdout(
      "
        Available recipes:
            foo:
                foo

            bar:
                bar
      ",
    )
    .run();
}

#[test]
fn module_recipe_list_alignment_ignores_private_recipes() {
  Test::new()
    .write(
      "foo.just",
      "
# foos
foo:
 @echo FOO

[private]
barbarbar:
 @echo BAR

@_bazbazbaz:
 @echo BAZ
      ",
    )
    .justfile("mod foo")
    .test_round_trip(false)
    .arg("--unstable")
    .arg("--list")
    .stdout(
      "
        Available recipes:
            foo:
                foo # foos
      ",
    )
    .run();
}

#[test]
fn nested_modules_are_properly_indented() {
  Test::new()
    .write("foo.just", "mod bar")
    .write("bar.just", "baz:\n @echo FOO")
    .justfile(
      "
        mod foo
      ",
    )
    .test_round_trip(false)
    .arg("--unstable")
    .arg("--list")
    .stdout(
      "
      Available recipes:
          foo:
              bar:
                  baz
    ",
    )
    .run();
}

#[test]
fn unsorted_list_order() {
  Test::new()
    .write("a.just", "a:")
    .write("b.just", "b:")
    .write("c.just", "c:")
    .write("d.just", "d:")
    .justfile(
      "
        import 'a.just'
        import 'b.just'
        import 'c.just'
        import 'd.just'
        x:
        y:
        z:
      ",
    )
    .args(["--list", "--unsorted"])
    .stdout(
      "
        Available recipes:
            x
            y
            z
            a
            b
            c
            d
      ",
    )
    .run();

  Test::new()
    .write("a.just", "a:")
    .write("b.just", "b:")
    .write("c.just", "c:")
    .write("d.just", "d:")
    .justfile(
      "
        x:
        y:
        z:
        import 'd.just'
        import 'c.just'
        import 'b.just'
        import 'a.just'
      ",
    )
    .args(["--list", "--unsorted"])
    .stdout(
      "
        Available recipes:
            x
            y
            z
            d
            c
            b
            a
      ",
    )
    .run();

  Test::new()
    .write("a.just", "a:\nimport 'e.just'")
    .write("b.just", "b:\nimport 'f.just'")
    .write("c.just", "c:\nimport 'g.just'")
    .write("d.just", "d:\nimport 'h.just'")
    .write("e.just", "e:")
    .write("f.just", "f:")
    .write("g.just", "g:")
    .write("h.just", "h:")
    .justfile(
      "
        x:
        y:
        z:
        import 'd.just'
        import 'c.just'
        import 'b.just'
        import 'a.just'
      ",
    )
    .args(["--list", "--unsorted"])
    .stdout(
      "
        Available recipes:
            x
            y
            z
            d
            h
            c
            g
            b
            f
            a
            e
      ",
    )
    .run();

  Test::new()
    .write("task1.just", "task1:")
    .write("task2.just", "task2:")
    .justfile(
      "
        import 'task1.just'
        import 'task2.just'
      ",
    )
    .args(["--list", "--unsorted"])
    .stdout(
      "
        Available recipes:
            task1
            task2
      ",
    )
    .run();
}
