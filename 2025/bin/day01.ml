let rotate curr command =
  let dir = command.[0] in
  let n = int_of_string (String.sub command 1 (String.length command - 1)) in
  match dir with
  | 'L' ->
      let res = (curr - n) mod 100 in
      if res < 0 then Some (res + 100, (((100 - curr) mod 100) + n) / 100)
      else Some (res, (((100 - curr) mod 100) + n) / 100)
  | 'R' -> Some ((curr + n) mod 100, (curr + n) / 100)
  | _ -> None

let () =
  let _, part1, part2 =
    Utils.read_lines "inputs/01.txt"
    |> List.fold_left
         (fun (pos, count0, count) line ->
           let npos, c = rotate pos line |> Option.get in
           let add0 = if npos == 0 then 1 else 0 in
           (npos, count0 + add0, count + c))
         (50, 0, 0)
  in

  Printf.printf "Part1: %d\nPart2: %d\n" part1 part2
