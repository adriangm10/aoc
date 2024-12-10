let trailhead_score_and_rating map y x =
  let dirs = [ (1, 0); (0, 1); (-1, 0); (0, -1) ] in

  let visited = Hashtbl.create 10 in

  let rec aux y x =
    if map.(y).(x) = 9 then
      Hashtbl.replace visited (y, x)
        ((Hashtbl.find_opt visited (y, x) |> Option.value ~default:0) + 1)
    else
      List.iter
        (fun (dy, dx) ->
          try
            if map.(y + dy).(x + dx) - map.(y).(x) = 1 then aux (y + dy) (x + dx)
          with Invalid_argument _ -> ())
        dirs
  in

  aux y x;

  let score = Hashtbl.length visited in
  let rating = Hashtbl.to_seq_values visited |> Seq.fold_left ( + ) 0 in
  (score, rating)

let () =
  let map =
    Utils.read_lines "inputs/input_10.txt"
    |> List.map (fun l ->
           String.to_seq l |> Array.of_seq
           |> Array.map (fun x -> int_of_char x - 48))
    |> Array.of_list
  in

  let _, score, rating =
    Array.fold_left
      (fun (i, score, rating) l ->
        let _, s, r =
          Array.fold_left
            (fun (j, score, rating) d ->
              let score, rating =
                if d = 0 then
                  let s, r = trailhead_score_and_rating map i j in
                  (score + s, rating + r)
                else (score, rating)
              in
              (j + 1, score, rating))
            (0, 0, 0) l
        in
        (i + 1, score + s, rating + r))
      (0, 0, 0) map
  in

  Printf.printf "Part1: %d\nPart2: %d\n" score rating
