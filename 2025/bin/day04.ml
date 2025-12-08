let check_roll map y x =
  let rolls = ref 0 in

  for i = Int.max (y - 1) 0 to Int.min (y + 1) (Array.length map - 1) do
    for j = Int.max (x - 1) 0 to Int.min (x + 1) (Array.length map.(0) - 1) do
      if map.(i).(j) = '@' then rolls := !rolls + 1
    done
  done;

  !rolls < 5

let () =
  let input =
    Utils.read_lines "inputs/04.txt"
    |> List.map (fun s -> String.to_seq s |> Array.of_seq)
    |> Array.of_list
  in

  let total_rolls = ref 0 in
  Array.iteri
    (fun i arr ->
      Array.iteri
        (fun j c ->
          if c = '@' && check_roll input i j then
            total_rolls := !total_rolls + 1)
        arr)
    input;

  let part2_rolls = ref 0 in
  let changed = ref true in
  while !changed do
    changed := false;
    for i = 0 to Array.length input - 1 do
      for j = 0 to Array.length input.(0) - 1 do
        if input.(i).(j) = '@' && check_roll input i j then (
          part2_rolls := !part2_rolls + 1;
          changed := true;
          input.(i).(j) <- '.')
      done
    done
  done;

  Printf.printf "Part1: %d\nPart2: %d\n" !total_rolls !part2_rolls
