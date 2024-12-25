type operation_type = AND | OR | XOR

type operation = {
  name1 : string;
  name2 : string;
  op : operation_type;
  nres : string;
}

let parse_op op =
  match Utils.split_string ~pattern:" -> " op with
  | [ op; nres ] -> (
      match Utils.split_on ' ' op with
      | [ n1; op; n2 ] -> (
          match op with
          | "AND" -> Some { name1 = n1; name2 = n2; op = AND; nres }
          | "OR" -> Some { name1 = n1; name2 = n2; op = OR; nres }
          | "XOR" -> Some { name1 = n1; name2 = n2; op = XOR; nres }
          | _ -> None)
      | _ -> None)
  | _ -> None

let operate v1 v2 = function
  | AND -> v1 land v2
  | OR -> v1 lor v2
  | XOR -> v1 lxor v2

let rec simulate val_map = function
  | [] ->
      Hashtbl.to_seq_keys val_map
      |> Seq.filter (fun x -> String.starts_with ~prefix:"z" x)
  | { name1 = n1; name2 = n2; op; nres } :: tl -> (
      match Hashtbl.find_opt val_map n1 with
      | Some v1 -> (
          match Hashtbl.find_opt val_map n2 with
          | Some v2 ->
              Hashtbl.replace val_map nres (operate v1 v2 op);
              simulate val_map tl
          | None ->
              simulate val_map (tl @ [ { name1 = n1; name2 = n2; op; nres } ]))
      | None -> simulate val_map (tl @ [ { name1 = n1; name2 = n2; op; nres } ])
      )

let () =
  let inputs, ops =
    match
      Utils.read_file "inputs/input_24.txt" |> Utils.split_string ~pattern:"\n\n"
    with
    | [ ins; ops ] -> (ins, ops)
    | _ -> raise Exit
  in

  let val_map = Hashtbl.create 20 in

  Utils.split_on '\n' inputs
  |> List.iter (fun l ->
         match Utils.split_string ~pattern:": " l with
         | [ name; value ] -> Hashtbl.add val_map name (int_of_string value)
         | _ -> ());

  let ops = Utils.split_on '\n' ops |> List.filter_map parse_op in

  let zs = simulate val_map ops |> List.of_seq |> List.fast_sort compare in

  let res1, _ =
    List.fold_left
      (fun (acc, i) z -> (acc + (Hashtbl.find val_map z lsl i), i + 1))
      (0, 0) zs
  in

  Printf.printf "Part1: %d\n" res1
