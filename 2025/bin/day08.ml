let euclidean_dist (x1, y1, z1) (x2, y2, z2) =
  let dx = x1 -. x2 in
  let dy = y1 -. y2 in
  let dz = z1 -. z2 in
  sqrt ((dx *. dx) +. (dy *. dy) +. (dz *. dz))

let make_groups junctions n_groups =
  (* list of distances sorted in ascending order *)
  let dists =
    junctions
    |> List.mapi (fun i junct1 ->
        junctions
        |> List.mapi (fun j junct2 -> (euclidean_dist junct1 junct2, i, j)))
    |> List.flatten
    |> List.filter (fun (d, _, _) -> d <> 0.0)
    |> List.sort_uniq (fun (d1, f1, t1) (d2, f2, t2) ->
        if f1 == t2 then 0 else compare (d1, f1, t1) (d2, f2, t2))
  in

  (* List.iter (fun (d, f, t) -> Printf.printf "(%f, %d, %d) " d f t) dists;
  print_newline (); *)

  let merge_partitions partitions a b =
    let min, max = if a > b then (b, a) else (a, b) in
    Array.map_inplace (fun x -> if x = max then min else x) partitions
  in

  let rec aux dists partitions n_groups =
    if n_groups == 0 then partitions
    else
      let _, idx_from, idx_to = List.hd dists in
      merge_partitions partitions partitions.(idx_from) partitions.(idx_to);
      aux (List.tl dists) partitions (n_groups - 1)
  in

  let partitions = Array.init (List.length junctions) (fun i -> i) in

  aux dists partitions n_groups

let () =
  let junctions =
    Utils.read_lines "inputs/08.txt"
    |> List.map (fun l ->
        match String.split_on_char ',' l with
        | [ x; y; z ] ->
            (float_of_string x, float_of_string y, float_of_string z)
        | _ -> raise (Failure "parse"))
  in

  let groups = make_groups junctions 1000 in

  let counts =
    Array.fold_left
      (fun acc g ->
        match List.assoc_opt g acc with
        | None -> (g, 1) :: acc
        | Some x -> (g, x + 1) :: List.remove_assoc g acc)
      [] groups
    |> List.sort (fun (_, c1) (_, c2) -> compare c2 c1)
  in

  (* List.iter (fun (_, c) -> Printf.printf "%d " c) counts;
  print_newline (); *)

  let (_, c1), (_, c2), (_, c3) =
    (List.hd counts, List.nth counts 1, List.nth counts 2)
  in

  Printf.printf "Part1: %d\n" (c1 * c2 * c3)
