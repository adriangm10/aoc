let max_area tiles =
  tiles
  |> List.map (fun (x1, y1) ->
      tiles
      |> List.fold_left
           (fun acc (x2, y2) ->
             let area = Int.abs (x1 - x2 + 1) * Int.abs (y1 - y2 + 1) in
             if area > acc then area else acc)
           (-1))
  |> List.fold_left (fun max area -> if area > max then area else max) (-1)

let () =
  let tiles =
    Utils.read_lines "inputs/09.txt"
    |> List.map (fun l ->
        match String.split_on_char ',' l with
        | [ x; y ] -> (int_of_string x, int_of_string y)
        | _ -> raise (Failure "parse"))
  in

  Printf.printf "Part1: %d\n" (max_area tiles)
