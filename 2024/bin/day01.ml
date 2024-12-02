(* x and y are sorted *)
let simmilarity x y =
  let rec aux x y sum last acc =
    match (x, y) with
    | [], [] -> acc
    | _ :: _, [] -> acc
    | [], _ :: _ -> acc
    | (hx :: tx as x), (hy :: ty as y) ->
        if hx < hy then
          if hx = last then aux tx y sum last ((sum * hx) :: acc)
          else aux tx y 0 (-1) acc
        else if hx = hy then
          if hx = last then aux x ty (sum + 1) hx acc else aux x ty 1 hx acc
        else aux x ty 0 (-1) acc
  in

  aux x y 0 (-1) [] |> List.fold_left ( + ) 0

let () =
  let x, y =
    Utils.read_lines "inputs/input_1.txt"
    |> List.map (fun l -> Utils.split_on ' ' l)
    |> List.map (fun x ->
           (int_of_string (List.nth x 0), int_of_string (List.nth x 1)))
    |> List.split
  in

  let x = List.sort compare x in
  let y = List.sort compare y in

  print_endline
    ("part1: "
    ^ string_of_int
        (List.map2 (fun x y -> Int.abs (x - y)) x y |> List.fold_left ( + ) 0));

  print_endline ("part2: " ^ string_of_int (simmilarity x y))
