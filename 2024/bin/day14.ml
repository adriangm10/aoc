let width = 101
let height = 103

(* you will have to find the ###### string on the output file to know the answer *)
let find_tree robots n =
  let out_file = open_out "output.txt" in
  let rec aux robots act =
    if act = n then close_out out_file
    else
      let mat = Array.init_matrix height width (fun _ _ -> '.') in
      List.iter (fun ((x, y), _) -> mat.(y).(x) <- '#') robots;
      Array.iter
        (fun l ->
          Printf.fprintf out_file "%s\n" (Array.to_seq l |> String.of_seq))
        mat;
      Printf.fprintf out_file "iteration: %d\n\n" act;

      let robots =
        List.map
          (fun ((x, y), (dx, dy)) ->
            let nx = (x + dx) mod width in
            let ny = (y + dy) mod height in
            let nx = if nx < 0 then nx + width else nx in
            let ny = if ny < 0 then ny + height else ny in
            ((nx, ny), (dx, dy)))
          robots
      in
      aux robots (act + 1)
  in

  aux robots 0

let () =
  let robots =
    Utils.read_lines "inputs/input_14.txt"
    |> List.filter_map (fun l ->
           match String.to_seq l |> Utils.numbers_in_line with
           | [ x; y; dx; dy ] -> Some ((x, y), (dx, dy))
           | _ -> None)
  in

  let q1, q2, q3, q4 =
    robots
    |> List.fold_left
         (fun (quad1, quad2, quad3, quad4) ((x, y), (dx, dy)) ->
           let nx = (x + (dx * 100)) mod width in
           let ny = (y + (dy * 100)) mod height in
           let nx = if nx < 0 then nx + width else nx in
           let ny = if ny < 0 then ny + height else ny in

           if nx < width / 2 && ny < height / 2 then
             (quad1 + 1, quad2, quad3, quad4)
           else if nx > width / 2 && ny < height / 2 then
             (quad1, quad2 + 1, quad3, quad4)
           else if nx < width / 2 && ny > height / 2 then
             (quad1, quad2, quad3 + 1, quad4)
           else if nx > width / 2 && ny > height / 2 then
             (quad1, quad2, quad3, quad4 + 1)
           else (quad1, quad2, quad3, quad4))
         (0, 0, 0, 0)
  in

  let res1 = q1 * q2 * q3 * q4 in
  find_tree robots 10_000;

  Printf.printf "Part1: %d\n" res1
