let has_repetition n =
  let nstr = string_of_int n in

  if String.length nstr mod 2 != 0 then false
  else
    let mid = String.length nstr / 2 in
    String.sub nstr 0 mid = String.sub nstr mid mid

let check_range x1 x2 =
  let rec aux x acc =
    if x > x2 then acc
    else if has_repetition x then aux (x + 1) (acc + x)
    else aux (x + 1) acc
  in

  aux x1 0

let () =
  let sol =
    Utils.read_all "inputs/02.txt"
    |> String.split_on_char ','
    |> List.fold_left
         (fun a range ->
           match String.split_on_char '-' range with
           | [ x1; x2 ] -> a + check_range (int_of_string x1) (int_of_string x2)
           | _ -> a)
         0
  in

  print_int sol
