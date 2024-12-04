let rec check_xmas letters (y, x) (dy, dx) next_c =
  try
    match (letters.(y).(x), next_c) with
    | 'X', 'X' -> check_xmas letters (y + dy, x + dx) (dy, dx) 'M'
    | 'M', 'M' -> check_xmas letters (y + dy, x + dx) (dy, dx) 'A'
    | 'A', 'A' -> check_xmas letters (y + dy, x + dx) (dy, dx) 'S'
    | 'S', 'S' -> true
    | _, _ -> false
  with Invalid_argument _ -> false

let count_xmas letters =
  let dirs =
    [| (0, 1); (1, 0); (1, 1); (0, -1); (-1, 0); (-1, -1); (-1, 1); (1, -1) |]
  in

  Array.mapi
    (fun i l ->
      Array.mapi
        (fun j _ ->
          Array.fold_left
            (fun acc (dy, dx) ->
              let xmas = check_xmas letters (i, j) (dy, dx) 'X' in
              acc + Bool.to_int xmas)
            0 dirs)
        l
      |> Array.fold_left ( + ) 0)
    letters
  |> Array.fold_left ( + ) 0

let check_x_mas letters (y, x) =
  try
    match letters.(y).(x) with
    | 'S' ->
        if check_xmas letters (y + 2, x + 2) (-1, -1) 'M' then
          match letters.(y).(x + 2) with
          | 'S' -> check_xmas letters (y + 2, x) (-1, 1) 'M'
          | 'M' -> check_xmas letters (y, x + 2) (1, -1) 'M'
          | _ -> false
        else false
    | 'M' ->
        if check_xmas letters (y, x) (1, 1) 'M' then
          match letters.(y).(x + 2) with
          | 'S' -> check_xmas letters (y + 2, x) (-1, 1) 'M'
          | 'M' -> check_xmas letters (y, x + 2) (1, -1) 'M'
          | _ -> false
        else false
    | _ -> false
  with Invalid_argument _ -> false

let count_x_mas letters =
  Array.mapi
    (fun i l ->
      Array.mapi (fun j _ -> Bool.to_int (check_x_mas letters (i, j))) l
      |> Array.fold_left ( + ) 0)
    letters
  |> Array.fold_left ( + ) 0

let () =
  let input =
    Utils.read_lines "inputs/input_4.txt"
    |> List.map (fun l -> Array.of_seq (String.to_seq l))
    |> Array.of_list
  in

  Printf.printf "part1: %d \npart2: %d\n" (count_xmas input) (count_x_mas input)
