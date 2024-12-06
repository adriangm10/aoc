module VisitedSet = Set.Make (struct
  type t = int * int

  let compare = compare
end)

module VisitedDirSet = Set.Make (struct
  type t = int * int * int * int

  let compare = compare
end)

let next_dir = function
  | -1, 0 -> (0, 1)
  | 0, 1 -> (1, 0)
  | 1, 0 -> (0, -1)
  | _ -> (-1, 0)

let check_loop map pos =
  let rec aux map (y, x) (dy, dx) visited =
    try
      match map.(y + dy).(x + dx) with
      | '#' ->
          if VisitedDirSet.mem (y + dy, x + dx, dy, dx) visited then 1
          else
            aux map (y, x)
              (next_dir (dy, dx))
              (VisitedDirSet.add (y + dy, x + dx, dy, dx) visited)
      | _ -> aux map (y + dy, x + dx) (dy, dx) visited
    with Invalid_argument _ -> 0
  in
  aux map pos (-1, 0) VisitedDirSet.empty

let walk_map map pos =
  let rec aux (y, x) (dy, dx) visited =
    try
      match map.(y + dy).(x + dx) with
      | '#' -> aux (y, x) (next_dir (dy, dx)) visited
      | _ -> aux (y + dy, x + dx) (dy, dx) (VisitedSet.add (y, x) visited)
    with Invalid_argument _ -> visited
  in

  let visited = aux pos (-1, 0) VisitedSet.empty in
  let loops =
    List.map
      (fun (y, x) ->
        map.(y).(x) <- '#';
        let loop = check_loop map pos in
        map.(y).(x) <- '.';
        loop)
      (VisitedSet.to_list visited)
    |> List.fold_left ( + ) 0
  in
  (VisitedSet.cardinal visited + 1, loops)

let find_init_pos map =
  let rec aux row =
    let idx = Array.find_index (fun x -> x = '^') map.(row) in
    match idx with Some i -> (row, i) | None -> aux (row + 1)
  in

  aux 0

let () =
  let map =
    Utils.read_lines "inputs/input_6.txt"
    |> List.map (fun l -> String.to_seq l |> Array.of_seq)
    |> Array.of_list
  in

  let y, x = find_init_pos map in

  let res1, res2 = walk_map map (y, x) in

  Printf.printf "part1: %d\npart2: %d\n" res1 res2
