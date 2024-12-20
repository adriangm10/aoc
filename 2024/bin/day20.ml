let traverse map =
  let start = Utils.find_pos map 'S' in
  let exit = Utils.find_pos map 'E' in

  let new_map =
    Array.init_matrix (Array.length map) (Array.length map.(0)) (fun _ _ -> -1)
  in

  let possible_dirs = function
    | 1, 0 -> [ (1, 0); (0, 1); (0, -1) ]
    | 0, 1 -> [ (0, 1); (1, 0); (-1, 0) ]
    | -1, 0 -> [ (-1, 0); (0, 1); (0, -1) ]
    | 0, -1 -> [ (0, -1); (1, 0); (-1, 0) ]
    | _ -> [ (0, 1); (0, -1); (1, 0); (-1, 0) ]
  in

  let get_next (y, x) dir =
    possible_dirs dir
    |> List.filter_map (fun (dy, dx) ->
           if map.(y + dy).(x + dx) <> '#' then Some ((y + dy, x + dx), (dy, dx))
           else None)
  in

  let rec aux (y, x) (dy, dx) step =
    new_map.(y).(x) <- step;
    if (y, x) = exit then ()
    else
      let npos, ndir = get_next (y, x) (dy, dx) |> List.hd in
      aux npos ndir (step + 1)
  in

  aux start (-1, -1) 0;
  new_map

let manhattan_distance (a, b) (c, d) = abs (c - a) + abs (d - b)

let count_cheats tr_map time =
  let height, width = (Array.length tr_map, Array.length tr_map.(0)) in

  let cheat_count = Hashtbl.create 1000 in

  let rec cheats_from (i, j) (y, x) =
    let d = manhattan_distance (i, j) (y, x) in
    if y > i + time || y > height - 1 then ()
    else if x > j + time || x > width - 1 then
      cheats_from (i, j) (y + 1, max 0 (j - time))
    else if d <= time && tr_map.(y).(x) >= 0 && tr_map.(y).(x) > tr_map.(i).(j)
    then (
      let n = abs (tr_map.(i).(j) - tr_map.(y).(x)) - d in
      Hashtbl.replace cheat_count n
        ((Hashtbl.find_opt cheat_count n |> Option.value ~default:0) + 1);
      cheats_from (i, j) (y, x + 1))
    else cheats_from (i, j) (y, x + 1)
  in

  Array.iteri
    (fun i arr ->
      Array.iteri
        (fun j s ->
          if s >= 0 then cheats_from (i, j) (max 0 (i - time), max 0 (j - time)))
        arr)
    tr_map;
  cheat_count

let () =
  let map =
    Utils.read_lines "inputs/input_20.txt"
    |> List.map (fun l -> String.to_seq l |> Array.of_seq)
    |> Array.of_list
  in

  let traversed_map = traverse map in

  let res1 =
    Hashtbl.fold
      (fun k v acc -> if k >= 100 then acc + v else acc)
      (count_cheats traversed_map 2)
      0
  in

  let res2 =
    Hashtbl.fold
      (fun k v acc -> if k >= 100 then acc + v else acc)
      (count_cheats traversed_map 20)
      0
  in

  Printf.printf "Part1: %d\nPart2: %d\n" res1 res2
