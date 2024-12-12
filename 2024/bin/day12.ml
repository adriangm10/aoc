let fence_price ?(discount = false) garden =
  let visited = Hashtbl.create 10 in
  let dirs = [ (1, 0); (0, 1); (-1, 0); (0, -1) ] in
  let maxy = Array.length garden in
  let maxx = Array.length garden.(0) in

  let rec no_disc (y, x) area perim =
    let c = garden.(y).(x) in

    List.fold_left
      (fun (area, perim) (dy, dx) ->
        let ny, nx = (y + dy, x + dx) in
        if ny >= 0 && nx >= 0 && ny < maxy && nx < maxx && garden.(ny).(nx) = c
        then
          if not (Hashtbl.mem visited (ny, nx)) then (
            Hashtbl.add visited (ny, nx) ();
            no_disc (ny, nx) (area + 1) perim)
          else (area, perim)
        else (area, 1 + perim))
      (area, perim) dirs
  in

  let prev_fence_dir = function
    | dir when dir = (1, 0) || dir = (-1, 0) -> ((0, 1), (0, -1))
    | _ -> ((1, 0), (-1, 0))
  in

  let rec disc queue area cont_fences fences =
    match queue with
    | [] -> (area, cont_fences)
    | (y, x) :: tl ->
        let c = garden.(y).(x) in
        let new_fences, npos =
          List.fold_left_map
            (fun new_fences (dy, dx) ->
              let ny, nx = (y + dy, x + dx) in
              if
                ny >= 0 && nx >= 0 && ny < maxy && nx < maxx
                && garden.(ny).(nx) = c
              then
                if not (Hashtbl.mem visited (ny, nx)) then (
                  Hashtbl.add visited (ny, nx) ();
                  (new_fences, Some (ny, nx)))
                else (new_fences, None)
              else (
                Hashtbl.add fences (y, x) (dy, dx);
                let (dy1, dx1), (dy2, dx2) = prev_fence_dir (dy, dx) in
                if
                  Hashtbl.find_all fences (y + dy1, x + dx1)
                  |> List.exists (fun dir -> dir = (dy, dx))
                  || Hashtbl.find_all fences (y + dy2, x + dx2)
                     |> List.exists (fun dir -> dir = (dy, dx))
                then (new_fences, None)
                else (new_fences + 1, None)))
            0 dirs
        in

        let npos = List.filter_map (fun x -> x) npos in

        disc (tl @ npos) (area + 1) (cont_fences + new_fences) fences
  in

  let _, price =
    Array.fold_left
      (fun (i, price) l ->
        let _, p =
          Array.fold_left
            (fun (j, p) _ ->
              if not (Hashtbl.mem visited (i, j)) then (
                Hashtbl.add visited (i, j) ();
                let a, perim =
                  if discount then disc [ (i, j) ] 0 0 (Hashtbl.create 10)
                  else no_disc (i, j) 1 0
                in
                (j + 1, p + (a * perim)))
              else (j + 1, p))
            (0, 0) l
        in
        (i + 1, price + p))
      (0, 0) garden
  in
  price

(* there should be a way of detecting the edges of the area and calculate its perimeter and number of sides *)
(* let _price ?(_discount = false) garden =
   let visited = Hashtbl.create 11 in
   let dirs = [ (1, 0); (0, 1); (-1, 0); (0, -1) ] in
   let maxy = Array.length garden in
   let maxx = Array.length garden.(0) in

   let is_corner (y, x) =
     let c = garden.(y).(x) in
     List.fold_left
       (fun acc (dy, dx) ->
         let ny, nx = (y + dy, x + dx) in
         if ny >= 0 && nx >= 0 && ny < maxy && nx < maxx && garden.(ny).(nx) = c
         then acc
         else acc + 1)
       0 dirs
     >= 2
   in

   let rec corners_area (y, x) corners area =
     let c = garden.(y).(x) in
     let corners = if is_corner (y, x) then (y, x) :: corners else corners in
     List.fold_left
       (fun (corners, area) (dy, dx) ->
         let ny, nx = (y + dy, x + dx) in
         if ny >= 0 && nx >= 0 && ny < maxy && nx < maxx && garden.(ny).(nx) = c
         then corners_area (ny, nx) corners (area + 1)
         else (corners, area))
       (corners, area) dirs
   in

   Array.fold_left
     (fun (i, price) l ->
       let _, p =
         Array.fold_left
           (fun (j, p) _ ->
             if not (Hashtbl.mem visited (i, j)) then (
               let corners, area = corners_area (i, j) [] 0 in
               print_string "corners: ";
               List.iter (fun (y, x) -> Printf.printf "(%d, %d); " y x) corners;
               print_newline ();
               (j + 1, p + area))
             else (j + 1, p))
           (0, 0) l
       in
       (i + 1, price + p))
     (0, 0) garden *)

let () =
  let garden =
    Utils.read_lines "inputs/input_12.txt"
    |> List.map (fun l -> String.to_seq l |> Array.of_seq)
    |> Array.of_list
  in

  let res1 = fence_price garden in
  let res2 = fence_price ~discount:true garden in
  Printf.printf "Part1: %d\nPart2: %d\n" res1 res2
