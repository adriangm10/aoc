let eval a b = function
  | '+' -> a + b
  | '*' -> a * b
  | _ -> int_of_string (string_of_int a ^ string_of_int b)

let is_possible res nums ops =
  let rec aux nums acc =
    if acc > res then 0
    else
      match nums with
      | hd :: tl ->
          ops
          |> List.map (eval acc hd)
          |> List.map (aux tl)
          |> List.fold_left (fun acc x -> if acc = 0 then x else acc) 0
      | [] when acc = res -> res
      | _ -> 0
  in

  aux nums 0

let () =
  let results, nums =
    Utils.read_lines "inputs/input_7.txt"
    |> List.map (fun l ->
           let eq = Utils.split_on ':' l in
           let nums =
             Utils.split_on ' ' (List.nth eq 1) |> List.map int_of_string
           in
           (int_of_string (List.nth eq 0), nums))
    |> List.split
  in

  let res1 =
    List.map2 (fun res nums -> is_possible res nums [ '+'; '*' ]) results nums
    |> List.fold_left ( + ) 0
  in

  let res2 =
    List.map2
      (fun res nums -> is_possible res nums [ '+'; '*'; '|' ])
      results nums
    |> List.fold_left ( + ) 0
  in

  Printf.printf "part1: %d\npart2: %d\n" res1 res2
