module CompSet = Set.Make (String)
module AddedSet = Set.Make (CompSet)

let find_tclique3 graph =
  let rec aux queue acc added start =
    match queue with
    | [] -> (acc, added)
    | (comp, set, n) :: tl when comp = start && n <> 3 ->
        if n = 0 && not (AddedSet.mem set added) then
          let added = AddedSet.add set added in
          aux tl (acc + 1) added start
        else aux tl acc added start
    | (comp, set, n) :: tl ->
        if n > 0 then
          let neighs =
            Hashtbl.find graph comp
            |> List.filter_map (fun c -> Some (c, CompSet.add comp set, n - 1))
          in
          aux (tl @ neighs) acc added start
        else aux tl acc added start
  in

  Hashtbl.to_seq_keys graph
  |> Seq.fold_left
       (fun (acc, added) comp ->
         if String.starts_with ~prefix:"t" comp then
           let cont, added = aux [ (comp, CompSet.empty, 3) ] 0 added comp in
           (acc + cont, added)
         else (acc, added))
       (0, AddedSet.empty)

(* this is not a deterministic algorithm, as the number of
   computers in the graph grow the probability of this
   algorithm not working decreases since they have to be
   ordered in a determined way for it to not work
   (for this type of graph) *)
let max_clique graph =
  let is_connected set v =
    let neighs = Hashtbl.find graph v in
    CompSet.for_all (fun c -> List.mem c neighs) set
  in

  let rec aux clique vs =
    match vs () with
    | Seq.Nil -> clique
    | Seq.Cons (v, seq) ->
        if is_connected clique v then aux (CompSet.add v clique) seq
        else aux clique seq
  in

  let vs = Hashtbl.to_seq_keys graph in

  vs
  |> Seq.fold_left
       (fun max_clique comp ->
         let clique = aux (CompSet.singleton comp) vs in
         if CompSet.cardinal clique > CompSet.cardinal max_clique then clique
         else max_clique)
       CompSet.empty

let () =
  let graph = Hashtbl.create ~random:true 20 in

  Utils.read_lines "inputs/input_23.txt"
  |> List.iter (fun l ->
         match Utils.split_on '-' l with
         | [ a; b ] ->
             Hashtbl.replace graph a
               (b :: (Hashtbl.find_opt graph a |> Option.value ~default:[]));
             Hashtbl.replace graph b
               (a :: (Hashtbl.find_opt graph b |> Option.value ~default:[]))
         | _ -> ());

  let res1, _ = find_tclique3 graph in
  let res2 = max_clique graph in

  Printf.printf "Part1: %d\nPart2: %s\n" res1
    (String.concat "," (CompSet.to_list res2))
