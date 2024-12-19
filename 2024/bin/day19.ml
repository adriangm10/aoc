let count_possibilities towel patterns =
  let n = String.length towel in
  let mem = Hashtbl.create 10 in

  let rec match_pattern i j pat =
    if j >= String.length pat then true
    else if i + j >= n || towel.[i + j] <> pat.[j] then false
    else match_pattern i (j + 1) pat
  in

  let rec aux i =
    match Hashtbl.find_opt mem i with
    | Some count -> count
    | None ->
        let count =
          if i >= n then 1
          else
            Hashtbl.find_all patterns towel.[i]
            |> List.filter (match_pattern i 0)
            |> List.fold_left (fun a pat -> aux (i + String.length pat) + a) 0
        in
        Hashtbl.add mem i count;
        count
  in

  aux 0

let () =
  let patterns, designs =
    match
      Utils.read_file "inputs/input_19.txt" |> Utils.split_string ~pattern:"\n\n"
    with
    | [ pat; des ] -> (pat, des)
    | _ -> raise Exit
  in

  let patts = Hashtbl.create 10 in

  Utils.split_string ~pattern:", " patterns
  |> List.iter (fun p -> Hashtbl.add patts p.[0] p);

  let designs = Utils.split_on '\n' designs in

  let res1, res2 =
    List.fold_left
      (fun (a1, a2) design ->
        match count_possibilities design patts with
        | x when x > 0 -> (a1 + 1, a2 + x)
        | _ -> (a1, a2))
      (0, 0) designs
  in

  Printf.printf "Part1: %d\nPart2: %d\n" res1 res2
