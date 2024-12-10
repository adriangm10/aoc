module IdSet = Set.Make (Int)

type block = Free of int | File of int * int

let checksum i (id, s) =
  let rec aux j acc = if j = i + s then acc else aux (j + 1) (acc + (j * id)) in
  aux i 0

let checksum_frag map =
  let files_rev =
    map |> List.filter (function Free _ -> false | File _ -> true) |> List.rev
  in

  let rec aux map files_rev (i, acc) =
    match (map, files_rev) with
    | _, [] -> acc
    | [], _ -> acc
    | File (id1, _) :: _, File (id2, s) :: _ when id1 = id2 ->
        acc + checksum i (id1, s)
    | File (id, s) :: mtl, fs -> aux mtl fs (i + s, acc + checksum i (id, s))
    | Free free_size :: mtl, File (id, s) :: ftl when free_size = s ->
        aux mtl ftl (i + s, acc + checksum i (id, s))
    | Free free_size :: mtl, File (id, s) :: ftl when free_size > s ->
        let free = Free (free_size - s) in
        aux (free :: mtl) ftl (i + s, acc + checksum i (id, s))
    | Free free_size :: mtl, File (id, s) :: ftl when free_size < s ->
        let rem_file = File (id, s - free_size) in
        aux mtl (rem_file :: ftl)
          (i + free_size, acc + checksum i (id, free_size))
    | _, _ -> raise Exit
  in

  aux map files_rev (0, 0)

let checksum_whole map =
  let files_rev =
    map |> List.filter (function Free _ -> false | File _ -> true) |> List.rev
  in

  let find_fit free_size files added =
    let rec aux = function
      | [] -> None
      | (File (id, s) as f) :: _ when s <= free_size && not (IdSet.mem id added)
        ->
          Some f
      | _ :: tl -> aux tl
    in

    aux files
  in

  let rec aux map (i, acc) added =
    match map with
    | [] -> acc
    | File (id, size) :: mtl ->
        if IdSet.mem id added then aux mtl (i + size, acc) added
        else
          aux mtl (i + size, acc + checksum i (id, size)) (IdSet.add id added)
    | Free free_size :: mtl -> (
        match find_fit free_size files_rev added with
        | None -> aux mtl (i + free_size, acc) added
        | Some (File (id, s)) ->
            let acc, added =
              if IdSet.mem id added then (acc, added)
              else (acc + checksum i (id, s), IdSet.add id added)
            in
            if s < free_size then
              aux (Free (free_size - s) :: mtl) (i + s, acc) added
            else aux mtl (i + s, acc) added
        | _ -> raise Exit)
  in

  aux map (0, 0) IdSet.empty

let () =
  let map =
    Utils.read_all "inputs/input_9.txt"
    |> String.fold_left
         (fun acc c ->
           let i = List.length acc in
           if i mod 2 = 0 then File (i / 2, int_of_char c - 48) :: acc
           else Free (int_of_char c - 48) :: acc)
         []
    |> List.rev
  in

  let res1 = checksum_frag map in
  let res2 = checksum_whole map in

  Printf.printf "part1: %d\npart2: %d\n" res1 res2
