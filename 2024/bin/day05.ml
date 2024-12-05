let is_correct (rules : (string, string) Hashtbl.t) (update : string list) =
  let rec aux update acc =
    match (update, acc) with
    | [], acc -> acc
    | hd :: tl, true ->
        aux tl
          (not
             (List.find_opt
                (fun x ->
                  Hashtbl.find_all rules x
                  |> List.find_opt (fun y -> hd = y)
                  |> Option.is_some)
                tl
             |> Option.is_some))
    | _, false -> false
  in

  aux update true

let correct_update rules update =
  let upd = Array.of_list update in
  let rec aux (update : string list) acc =
    match update with
    | [] -> None
    | hd :: tl -> (
        let idx =
          List.find_index
            (fun x ->
              Hashtbl.find_all rules x
              |> List.find_opt (fun y -> hd = y)
              |> Option.is_some)
            tl
        in

        match idx with
        | Some i -> Some (acc, i + acc + 1)
        | None -> aux tl (acc + 1))
  in

  let idxs = ref (aux (Array.to_list upd) 0) in
  while Option.is_some !idxs do
    let i, j = Option.value ~default:(-1, -1) !idxs in
    let a = upd.(i) in
    upd.(i) <- upd.(j);
    upd.(j) <- a;
    idxs := aux (Array.to_list upd) 0
  done;

  Array.to_list upd

let () =
  let rules, updates =
    Utils.read_lines "inputs/input_5.txt"
    |> List.partition (fun l -> String.contains l '|')
  in

  let rules =
    List.map (fun r -> Utils.split_on '|' r) rules
    |> List.fold_left
         (fun acc r ->
           Hashtbl.add acc (List.hd r) (List.nth r 1);
           acc)
         (Hashtbl.create (List.length rules))
  in

  let updates = List.map (fun u -> Utils.split_on ',' u) (List.tl updates) in

  let correct, incorrect =
    List.partition (fun u -> is_correct rules u) updates
  in

  let res1 =
    List.map (fun u -> List.length u / 2 |> List.nth u |> int_of_string) correct
    |> List.fold_left ( + ) 0
  in

  let res2 =
    List.map (fun u -> correct_update rules u) incorrect
    |> List.map (fun u -> List.length u / 2 |> List.nth u |> int_of_string)
    |> List.fold_left ( + ) 0
  in

  Printf.printf "part1: %d\npart2: %d\n" res1 res2
