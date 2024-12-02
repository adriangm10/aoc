let remove_n l n =
  let rec aux n acc = function
    | [] -> acc
    | hd :: tl ->
        if n = 0 then List.rev tl @ acc else aux (n - 1) (hd :: acc) tl
  in

  aux n [] l

let is_safe report =
  let dir = compare (List.nth report 0) (List.nth report 1) in

  let rec aux last = function
    | [] -> true
    | [ x ] ->
        let diff = last - x in
        if diff * dir <= 0 || Int.abs diff > 3 then false else true
    | x1 :: (x2 :: _ as tl) ->
        let diff = x1 - x2 in
        if diff * dir <= 0 || Int.abs diff > 3 then false else aux x1 tl
  in

  aux 0 report

let () =
  let reports =
    Utils.read_lines "inputs/input_2.txt"
    |> List.map (fun l -> List.map int_of_string (Utils.split_on ' ' l))
  in

  let res1 =
    List.fold_left (fun acc l -> acc + Bool.to_int (is_safe l)) 0 reports
  in

  let res2 =
    List.fold_left
      (fun acc r ->
        let safe = ref (is_safe r) in
        let i = ref 0 in
        while (not !safe) && !i < List.length r do
          let l = remove_n r !i in
          safe := is_safe l;
          i := !i + 1;
        done;
        acc + Bool.to_int !safe)
      0 reports
  in

  print_endline ("part1: " ^ string_of_int res1);
  print_endline ("part2: " ^ string_of_int res2)
