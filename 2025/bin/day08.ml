module IntegerSet = Set.Make (Int)

let euclidean_dist (x1, y1, z1) (x2, y2, z2) =
  let dx = x1 -. x2 in
  let dy = y1 -. y2 in
  let dz = z1 -. z2 in
  sqrt ((dx *. dx) +. (dy *. dy) +. (dz *. dz))

(* with a tree type partition these would be much faster *)
let merge_partitions partitions a b =
  let min, max = if a > b then (b, a) else (a, b) in
  Array.map_inplace (fun x -> if x = max then min else x) partitions

let count_partitions partitions =
  partitions
  |> Array.fold_left (fun set x -> IntegerSet.add x set) IntegerSet.empty
  |> IntegerSet.cardinal

let make_conns junctions dists n_conns =
  let rec aux dists partitions n_conns =
    if n_conns == 0 then partitions
    else
      let _, idx_from, idx_to = List.hd dists in
      if partitions.(idx_from) <> partitions.(idx_to) then
        merge_partitions partitions partitions.(idx_from) partitions.(idx_to);
      aux (List.tl dists) partitions (n_conns - 1)
  in

  let partitions = Array.init (List.length junctions) (fun i -> i) in

  aux dists partitions n_conns

let connect_all junctions dists =
  let rec aux dists partitions =
    let _, idx_from, idx_to = List.hd dists in
    if
      partitions.(idx_from) <> partitions.(idx_to)
      && count_partitions partitions == 2
    then List.hd dists
    else (
      merge_partitions partitions partitions.(idx_from) partitions.(idx_to);
      aux (List.tl dists) partitions)
  in

  let partitions = Array.init (List.length junctions) (fun i -> i) in

  aux dists partitions

let () =
  let junctions =
    Utils.read_lines "inputs/08.txt"
    |> List.map (fun l ->
        match String.split_on_char ',' l with
        | [ x; y; z ] ->
            (float_of_string x, float_of_string y, float_of_string z)
        | _ -> raise (Failure "parse"))
  in

  (* list of (dinstance, from, to) sorted in ascending order *)
  let dists =
    junctions
    |> List.mapi (fun i junct1 ->
        junctions
        |> List.mapi (fun j junct2 -> (euclidean_dist junct1 junct2, i, j)))
    |> List.flatten
    |> List.filter (fun (d, _, _) -> d <> 0.0)
    |> List.sort_uniq (fun (d1, f1, t1) (d2, f2, t2) ->
        if f1 == t2 && t1 == f2 then compare d1 d2
        else compare (d1, f1, t1) (d2, f2, t2))
  in

  let groups = make_conns junctions dists 1000 in
  let _, last_connf, last_connt = connect_all junctions dists in
  let x1, _, _ = List.nth junctions last_connf in
  let x2, _, _ = List.nth junctions last_connt in

  let counts =
    Array.fold_left
      (fun acc g ->
        match List.assoc_opt g acc with
        | None -> (g, 1) :: acc
        | Some x -> (g, x + 1) :: List.remove_assoc g acc)
      [] groups
    |> List.sort (fun (_, c1) (_, c2) -> compare c2 c1)
  in

  let (_, c1), (_, c2), (_, c3) =
    (List.hd counts, List.nth counts 1, List.nth counts 2)
  in

  Printf.printf "Part1: %d\nPart2: %.0f\n" (c1 * c2 * c3) (x1 *. x2)
