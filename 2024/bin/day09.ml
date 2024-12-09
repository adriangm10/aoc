type block = Free of int | File of int * int

module Block = struct
  let is_file = function Free _ -> false | File _ -> true
  (* let is_free = function Free _ -> true | File _ -> false *)

  let free_size = function
    | Free s -> s
    | File _ -> raise (Invalid_argument "is not Free")

  let file_size = function
    | Free _ -> raise (Invalid_argument "is not a file")
    | File (_, s) -> s

  let file_id = function
    | Free _ -> raise (Invalid_argument "is not a file")
    | File (id, _) -> id

  let get_file = function
    | Free _ -> raise (Invalid_argument "is not a file")
    | File (id, s) -> (id, s)
end

let rearrange map =
  let files_rev =
    map |> List.filter (function Free _ -> false | File _ -> true) |> List.rev
  in

  let rec aux map files_rev acc =
    match (map, files_rev) with
    | _, [] -> acc
    | [], _ -> acc
    | mhd :: _, fhd :: _
      when Block.is_file mhd && Block.file_id mhd = Block.file_id fhd ->
        fhd :: acc
    | mhd :: mtl, fs when Block.is_file mhd -> aux mtl fs (mhd :: acc)
    | mhd :: mtl, fhd :: ftl when Block.free_size mhd = Block.file_size fhd ->
        aux mtl ftl (fhd :: acc)
    | mhd :: mtl, fhd :: ftl when Block.free_size mhd > Block.file_size fhd ->
        let free = Free (Block.free_size mhd - Block.file_size fhd) in
        aux (free :: mtl) ftl (fhd :: acc)
    | mhd :: mtl, fhd :: ftl when Block.free_size mhd < Block.file_size fhd ->
        let id, size = Block.get_file fhd in
        let rem_file = File (id, size - Block.free_size mhd) in
        let ins_file = File (id, Block.free_size mhd) in
        aux mtl (rem_file :: ftl) (ins_file :: acc)
    | _, _ -> raise Exit
  in

  List.rev (aux map files_rev [])

let () =
  let map =
    Utils.read_all "inputs/test_9.txt"
    |> String.fold_left
         (fun acc c ->
           let i = List.length acc in
           if i mod 2 = 0 then File (i / 2, int_of_char c - 48) :: acc
           else Free (int_of_char c - 48) :: acc)
         []
    |> List.rev
  in

  rearrange map
  |> List.iter (fun b ->
         match b with
         | Free s -> Printf.printf "Free: %d\n" s
         | File (id, s) -> Printf.printf "File {id: %d, space: %d}\n" id s)
