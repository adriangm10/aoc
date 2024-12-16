module PathSet = Set.Make (struct
  type t = int * int

  let compare = compare
end)

let find_pos map c =
  let rec aux row =
    let idx = Array.find_index (fun x -> x = c) map.(row) in
    match idx with Some i -> (row, i) | None -> aux (row + 1)
  in

  aux 0

let find_path map (y, x) target =
  let visited = Hashtbl.create 10 in

  (* (ny, nx, dy, dx, cost) list *)
  let get_neighbours (y, x) facing =
    let dirs =
      match facing with
      | 0, 1 -> [ (0, 1, 1); (1, 0, 1001); (-1, 0, 1001) ]
      | 1, 0 -> [ (1, 0, 1); (0, 1, 1001); (0, -1, 1001) ]
      | 0, -1 -> [ (0, -1, 1); (1, 0, 1001); (-1, 0, 1001) ]
      | _ -> [ (-1, 0, 1); (0, 1, 1001); (0, -1, 1001) ]
    in

    List.fold_left
      (fun acc (dy, dx, v) ->
        match map.(y + dy).(x + dx) with
        | '#' -> acc
        | _ -> (y + dy, x + dx, dy, dx, v) :: acc)
      [] dirs
  in

  let rec aux queue (min_cost, seats) =
    match queue with
    | [] -> (min_cost, seats)
    | (y, x, _, _, cost, path) :: tl when (y, x) = target ->
        if cost < min_cost then aux tl (cost, path)
        else if cost = min_cost then aux tl (cost, PathSet.union path seats)
        else aux tl (min_cost, seats)
    | (y, x, dy, dx, cost, path) :: tl ->
        let neigh =
          get_neighbours (y, x) (dy, dx)
          |> List.filter_map (fun (y, x, dy, dx, c) ->
                 if
                   Hashtbl.find_all visited (y, x, dy, dx)
                   |> List.exists (fun cc -> cc < c + cost)
                 then None
                 else (
                   Hashtbl.add visited (y, x, dy, dx) (c + cost);
                   Some (y, x, dy, dx, c + cost, PathSet.add (y, x) path)))
        in
        aux (tl @ neigh) (min_cost, seats)
  in

  aux [ (y, x, 0, 1, 0, PathSet.singleton (y, x)) ] (Int.max_int, PathSet.empty)

let () =
  let map =
    Utils.read_lines "inputs/input_16.txt"
    |> List.map (fun l -> String.to_seq l |> Array.of_seq)
    |> Array.of_list
  in

  let init_pos = find_pos map 'S' in
  let target = find_pos map 'E' in
  let res1, best_seats = find_path map init_pos target in

  Printf.printf "Part1: %d\nPart2: %d\n" res1 (PathSet.cardinal best_seats)
