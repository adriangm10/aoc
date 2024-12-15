let find_init_pos map =
  let rec aux row =
    let idx = Array.find_index (fun x -> x = '@') map.(row) in
    match idx with Some i -> (row, i) | None -> aux (row + 1)
  in

  aux 0

let move_dir = function
  | '^' -> (-1, 0)
  | '>' -> (0, 1)
  | 'v' -> (1, 0)
  | '<' -> (0, -1)
  | _ -> (0, 0)

let move_bot (y, x) map move =
  let dy, dx = move_dir move in

  let rec aux (y, x) =
    let ny, nx = (y + dy, x + dx) in
    match map.(ny).(nx) with
    | o when o = 'O' || o = '[' || o = ']' ->
        if aux (ny, nx) then (
          map.(ny).(nx) <- map.(y).(x);
          map.(y).(x) <- '.';
          true)
        else false
    | '.' ->
        map.(ny).(nx) <- map.(y).(x);
        map.(y).(x) <- '.';
        true
    | _ -> false
  in

  if aux (y, x) then (y + dy, x + dx) else (y, x)

let move_in_dmap (y, x) map move =
  match move with
  | m when m = '<' || m = '>' -> move_bot (y, x) map move
  | _ ->
      let dy, dx = move_dir move in

      let rec check (y, x) =
        let ny, nx = (y + dy, x + dx) in
        match map.(ny).(nx) with
        | '[' -> check (ny, nx) && check (ny, nx + 1)
        | ']' -> check (ny, nx) && check (ny, nx - 1)
        | '.' -> true
        | _ -> false
      in

      let rec move (y, x) =
        let ny, nx = (y + dy, x + dx) in
        if map.(ny).(nx) = '[' then (
          move (ny, nx);
          move (ny, nx + 1))
        else if map.(ny).(nx) = ']' then (
          move (ny, nx);
          move (ny, nx - 1));

        if map.(ny).(x) = '.' then (
          map.(ny).(nx) <- map.(y).(x);
          map.(y).(x) <- '.')
      in

      if check (y, x) then (
        move (y, x);
        (y + dy, x + dx))
      else (y, x)

let double_map (map : char array array) =
  Array.fold_right
    (fun arr acc ->
      (Array.fold_right
         (fun c acc ->
           match c with
           | '#' -> '#' :: '#' :: acc
           | 'O' -> '[' :: ']' :: acc
           | '.' -> '.' :: '.' :: acc
           | _ -> '@' :: '.' :: acc)
         arr []
      |> Array.of_list)
      :: acc)
    map []
  |> Array.of_list

let () =
  let map, moves =
    match
      Utils.read_file "inputs/input_15.txt"
      |> Utils.split_string ~pattern:"\n\n"
    with
    | [ map; moves ] -> (map, moves)
    | _ -> raise Exit
  in

  let map =
    String.split_on_char '\n' map
    |> List.map (fun l -> l |> String.to_seq |> Array.of_seq)
    |> Array.of_list
  in

  let dmap = double_map map in

  let pos = ref (find_init_pos map) in
  let y, x = !pos in
  let dpos = ref (y, x * 2) in

  String.iter (fun c -> pos := move_bot !pos map c) moves;
  String.iter (fun c -> dpos := move_in_dmap !dpos dmap c) moves;

  let _, res1 =
    Array.fold_left
      (fun (i, acc) arr ->
        let _, a =
          Array.fold_left
            (fun (j, acc) c ->
              if c = 'O' then (j + 1, acc + (100 * i) + j) else (j + 1, acc))
            (0, 0) arr
        in
        (i + 1, acc + a))
      (0, 0) map
  in

  let _, res2 =
    Array.fold_left
      (fun (i, acc) arr ->
        let _, a =
          Array.fold_left
            (fun (j, acc) c ->
              if c = '[' then (j + 1, acc + (100 * i) + j) else (j + 1, acc))
            (0, 0) arr
        in
        (i + 1, acc + a))
      (0, 0) dmap
  in

  Printf.printf "Part1: %d\nPart2: %d\n" res1 res2
