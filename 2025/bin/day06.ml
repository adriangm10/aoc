let rec transpose = function
  | [] | [] :: _ -> []
  | rows -> List.map List.hd rows :: transpose (List.map List.tl rows)

let operate nums op =
  let op_fun, init =
    match op with
    | "*" -> (( * ), 1)
    | "+" -> (( + ), 0)
    | _ -> raise (Failure "operate")
  in

  List.fold_left op_fun init nums

let parse1 file =
  let nums, ops =
    Utils.read_lines file
    |> List.partition_map (fun s ->
        try Left (Utils.split_on ' ' s |> List.map int_of_string)
        with _ -> Right (Utils.split_on ' ' s))
  in

  (transpose nums, List.nth ops 0)

let parse2 file =
  let rec split_on_none_row curr acc = function
    | [] -> if curr == [] then acc else curr :: acc
    | row :: t when List.for_all Option.is_none row ->
        split_on_none_row [] (if curr == [] then acc else curr :: acc) t
    | row :: t -> split_on_none_row (row :: curr) acc t
  in

  let rec parse_num acc = function
    | [] -> acc
    | None :: t -> parse_num acc t
    | Some x :: t -> parse_num ((acc * 10) + x) t
  in

  let input = Utils.read_lines file in
  let nums =
    List.take (List.length input - 1) input
    |> List.map (fun s ->
        String.to_seq s
        |> Seq.map (fun c ->
            if Utils.is_digit c then Some (int_of_char c - int_of_char '0')
            else None)
        |> List.of_seq)
    |> transpose |> split_on_none_row [] []
    |> List.map (fun x -> List.map (parse_num 0) x)
  in

  let ops = List.nth input (List.length input - 1) |> Utils.split_on ' ' in

  (nums, List.rev ops)

let () =
  let nums1, ops1 = parse1 "inputs/06.txt" in
  let nums2, ops2 = parse2 "inputs/06.txt" in

  let res1 =
    List.map2 (fun ns op -> operate ns op) nums1 ops1 |> List.fold_left ( + ) 0
  in

  let res2 =
    List.map2 (fun ns op -> operate ns op) nums2 ops2 |> List.fold_left ( + ) 0
  in

  Printf.printf "Part1: %d\nPart2: %d\n" res1 res2
