type block = Free of int | File of int * int

let () =
  let input =
    Utils.read_all "inputs/test_9.txt"
    |> String.fold_left
         (fun acc c ->
           let i = List.length acc in
           if i mod 2 = 0 then File (i / 2, int_of_char c - 48) :: acc
           else Free (int_of_char c - 48) :: acc)
         []
    |> List.rev
  in

  List.iter
    (function
      | Free s -> Printf.printf "Free: %d\n" s
      | File (id, s) -> Printf.printf "File { id: %d, space: %d }\n" id s)
    input
