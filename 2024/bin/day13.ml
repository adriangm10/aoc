let parse_equation ~add_to_price lines =
  (* returns a list of (y, x) pairs *)
  let rec numbers_in_line acc act char_seq =
    match char_seq () with
    | Seq.Nil -> int_of_string act :: acc
    | Seq.Cons (c, seq) when Utils.is_digit c ->
        numbers_in_line acc (act ^ String.make 1 c) seq
    | Seq.Cons (_, seq) when act = "" -> numbers_in_line acc act seq
    | Seq.Cons (_, seq) -> numbers_in_line (int_of_string act :: acc) "" seq
  in

  match
    Utils.split_on '\n' lines
    |> List.map (fun l ->
           let l = String.to_seq l in
           let nums = numbers_in_line [] "" l in
           (List.hd nums, List.nth nums 1))
  with
  | [ (ay, ax); (by, bx); (py, px) ] ->
      Some
        [| [| ax; bx; px + add_to_price |]; [| ay; by; py + add_to_price |] |]
  | _ -> None

(* (2, 3) equation *)
let solve_eq eq =
  let solv_eq = Array.map (fun x -> Array.copy x) eq in
  solv_eq.(1).(1) <-
    (solv_eq.(1).(1) * solv_eq.(0).(0)) - (solv_eq.(0).(1) * solv_eq.(1).(0));
  solv_eq.(1).(2) <-
    (solv_eq.(1).(2) * solv_eq.(0).(0)) - (solv_eq.(0).(2) * solv_eq.(1).(0));

  let y = solv_eq.(1).(2) / solv_eq.(1).(1) in
  let x = (solv_eq.(0).(2) - (solv_eq.(0).(1) * y)) / solv_eq.(0).(0) in

  if
    (eq.(0).(0) * x) + (eq.(0).(1) * y) <> eq.(0).(2)
    || (eq.(1).(0) * x) + (eq.(1).(1) * y) <> eq.(1).(2)
  then None
  else Some (x, y)

let () =
  let input =
    Utils.read_file "inputs/input_13.txt" |> Utils.split_string ~pattern:"\n\n"
  in

  let input1 = input |> List.filter_map (parse_equation ~add_to_price:0) in

  let input2 =
    input |> List.filter_map (parse_equation ~add_to_price:10000000000000)
  in

  let res1 =
    List.fold_left
      (fun acc eq ->
        match solve_eq eq with
        | Some (x, y) -> acc + ((x * 3) + y)
        | None -> acc)
      0 input1
  in

  let res2 =
    List.fold_left
      (fun acc eq ->
        match solve_eq eq with
        | Some (x, y) -> acc + ((x * 3) + y)
        | None -> acc)
      0 input2
  in

  Printf.printf "Part1: %d\nPart2: %d\n" res1 res2
