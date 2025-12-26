let sol1 start splitters maxy =
  let rec split_beams acc n = function
    | [] -> (acc, n)
    | (y, x) :: tl ->
        let acc, n =
          if List.mem (y, x) splitters then
            let acc =
              if List.mem (y, x - 1) acc then acc else (y, x - 1) :: acc
            in
            if List.mem (y, x + 1) acc then (acc, n + 1)
            else ((y, x + 1) :: acc, n + 1)
          else if List.mem (y, x) acc then (acc, n)
          else ((y, x) :: acc, n)
        in
        split_beams acc n tl
  in

  let rec aux beams acc =
    let y1, _ = List.hd beams in
    if y1 >= maxy then acc
    else
      let beams, n =
        List.map (fun (y, x) -> (y + 1, x)) beams |> split_beams [] 0
      in
      aux beams (acc + n)
  in

  aux [ start ] 0

let sol2 start splitters maxy =
  let rec split_beams acc = function
    | [] -> acc
    | ((y, x), c) :: tl ->
        let acc =
          if List.mem (y, x) splitters then
            let acc =
              match List.assoc_opt (y, x - 1) acc with
              | None -> ((y, x - 1), c) :: acc
              | Some c1 ->
                  ((y, x - 1), c + c1) :: List.remove_assoc (y, x - 1) acc
            in

            match List.assoc_opt (y, x + 1) acc with
            | None -> ((y, x + 1), c) :: acc
            | Some c1 ->
                ((y, x + 1), c + c1) :: List.remove_assoc (y, x + 1) acc
          else
            match List.assoc_opt (y, x) acc with
            | None -> ((y, x), c) :: acc
            | Some c1 -> ((y, x), c + c1) :: List.remove_assoc (y, x) acc
        in
        split_beams acc tl
  in

  let rec aux beams =
    let (y1, _), _ = List.hd beams in
    if y1 >= maxy then beams
    else
      let beams =
        List.map (fun ((y, x), c) -> ((y + 1, x), c)) beams |> split_beams []
      in
      aux beams
  in

  aux [ (start, 1) ] |> List.fold_left (fun acc ((_, _), c) -> acc + c) 0

let () =
  let input = Utils.read_lines "inputs/07.txt" in
  let start =
    ( 0,
      List.hd input |> String.to_seq
      |> Seq.find_index (fun c -> c == 'S')
      |> Option.get )
  in

  let splitters =
    input
    |> List.mapi (fun i l ->
        String.to_seq l
        |> Seq.mapi (fun j c -> if c == '^' then j else -1)
        |> Seq.filter (fun j -> j <> -1)
        |> Seq.map (fun j -> (i, j))
        |> List.of_seq)
    |> List.flatten
  in

  let res1 = sol1 start splitters (List.length input) in
  let res2 = sol2 start splitters (List.length input) in

  Printf.printf "Part1: %d\nPart2: %d\n" res1 res2

(* Printf.printf "Start: (%d, %d)\n" s0 s1;

  List.iter (fun (x1, x2) -> Printf.printf "splitter: (%d, %d)\n" x1 x2) splitters *)
