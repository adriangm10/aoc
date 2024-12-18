let read_lines file_name =
  let ic = open_in file_name in
  let try_line () = try Some (input_line ic) with End_of_file -> None in

  let rec loop acc =
    match try_line () with
    | Some line -> loop (line :: acc)
    | None ->
        close_in ic;
        List.rev acc
  in

  loop []

let read_all file =
  let ic = open_in file in

  let rec loop acc =
    try
      let line = input_line ic in
      loop (acc ^ line)
    with End_of_file ->
      close_in ic;
      acc
  in

  loop ""

let read_file file =
  let ic = open_in file in

  let rec loop acc =
    try
      let line = input_line ic in
      loop (acc ^ line ^ "\n")
    with End_of_file ->
      close_in ic;
      acc
  in

  loop ""

let split_on c s = String.split_on_char c s |> List.filter (fun s -> s <> "")
let is_digit = function '0' .. '9' -> true | _ -> false

let split_string ~pattern str =
  let n = String.length str in
  let pat_len = String.length pattern in

  let rec match_pattern i j =
    if j >= pat_len then true
    else if i + j >= n || str.[i + j] <> pattern.[j] then false
    else match_pattern i (j + 1)
  in

  let rec aux i acc act =
    if i >= n then List.rev (act :: acc)
    else if match_pattern i 0 then aux (i + pat_len) (act :: acc) ""
    else aux (i + 1) acc (act ^ String.make 1 str.[i])
  in

  aux 0 [] ""

let print_matrix ~fin m =
  Array.iter
    (fun row ->
      Array.iter (fun elem -> Printf.printf "%d " elem) row;
      Printf.printf "\n")
    m;

  print_string fin

let numbers_in_string s =
  let rec aux acc act char_seq =
    match char_seq () with
    | Seq.Nil ->
        if act <> "" && act <> "-" then int_of_string act :: acc else acc
    | Seq.Cons (c, seq) when is_digit c || (c = '-' && act = "") ->
        aux acc (act ^ String.make 1 c) seq
    | Seq.Cons (_, seq) when act = "" || act = "-" -> aux acc "" seq
    | Seq.Cons (_, seq) -> aux (int_of_string act :: acc) "" seq
  in

  List.rev (aux [] "" (String.to_seq s))
