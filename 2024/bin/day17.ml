let simulate (a, b, c) prog =
  let combo_value (a, b, c) = function
    | n when n <= 3 -> n
    | 4 -> a
    | 5 -> b
    | _ -> c
  in

  let rec aux (a, b, c) ip out =
    try
      match prog.(ip) with
      | 0 ->
          let a =
            a
            / int_of_float
                (float_of_int 2
                ** float_of_int (combo_value (a, b, c) prog.(ip + 1)))
          in
          aux (a, b, c) (ip + 2) out
      | 1 ->
          let b = b lxor prog.(ip + 1) in
          aux (a, b, c) (ip + 2) out
      | 2 ->
          let b = combo_value (a, b, c) prog.(ip + 1) mod 8 in
          aux (a, b, c) (ip + 2) out
      | 3 ->
          if a = 0 then aux (a, b, c) (ip + 2) out
          else aux (a, b, c) prog.(ip + 1) out
      | 4 ->
          let b = b lxor c in
          aux (a, b, c) (ip + 2) out
      | 5 ->
          let outv = combo_value (a, b, c) prog.(ip + 1) mod 8 in
          let out = string_of_int outv :: out in
          aux (a, b, c) (ip + 2) out
      | 6 ->
          let b =
            a
            / int_of_float
                (float_of_int 2
                ** float_of_int (combo_value (a, b, c) prog.(ip + 1)))
          in
          aux (a, b, c) (ip + 2) out
      | _ ->
          let c =
            a
            / int_of_float
                (float_of_int 2
                ** float_of_int (combo_value (a, b, c) prog.(ip + 1)))
          in
          aux (a, b, c) (ip + 2) out
    with Invalid_argument _ -> out
  in

  List.rev (aux (a, b, c) 0 [])

let _correct prog =
  let rec gen_table acc i =
    if i < 8 then
      let v = simulate (i, 0, 0) prog |> List.hd |> int_of_string in
      let acc = if List.mem_assoc v acc then acc else (v, i) :: acc in
      gen_table acc (i + 1)
    else acc
  in

  let table = gen_table [ (7, 15); (6, 30); (4, 14) ] 0 in

  List.iter (fun (x, y) -> Printf.printf "%d, %d\n" x y) table;

  let rec aux acc prog =
    match prog with
    | [] -> acc
    | hd :: tl ->
        (* Printf.printf "%d * 8 ** %d + " hd (List.length prog); *)
        let pow =
          List.assoc hd table
          * (8. ** (List.length prog |> float_of_int) |> int_of_float)
        in
        aux (acc + pow) tl
  in

  aux 0 (Array.to_list prog |> List.rev)

let () =
  let regs, prog =
    match
      Utils.read_file "inputs/input_17.txt"
      |> Utils.split_string ~pattern:"\n\n"
    with
    | [ regs; prog ] -> (regs, prog)
    | _ -> raise Exit
  in

  let regs =
    match Utils.split_on '\n' regs |> List.map Utils.numbers_in_string with
    | [ [ a ]; [ b ]; [ c ] ] -> (a, b, c)
    | _ -> raise Exit
  in

  let prog = Utils.numbers_in_string prog |> Array.of_list in

  print_endline ("Part1: " ^ String.concat "," (simulate regs prog));
  (* let res2 = correct prog in
  Printf.printf "Part2: %d\n" res2;
  print_endline ("corrected: " ^ String.concat "," (simulate (res2, 0, 0) prog)); *)

  let rec loop i =
    if i < 100000 then (
      Printf.printf "%d: %s\n" i (String.concat "," (simulate (i, 0, 0) prog));
      loop (i + 1))
    else ()
  in

  loop 0
