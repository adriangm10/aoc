let width = 70
let height = 70
let bytes = 1024

module Path = Set.Make (struct
  type t = int * int

  let compare = compare
end)

let path_len obs =
  let target = (width, height) in
  let visited = Hashtbl.create 10 in

  let get_neighbours (x, y) =
    [ (x - 1, y); (x + 1, y); (x, y - 1); (x, y + 1) ]
  in

  let rec aux = function
    | [] -> None
    | (x, y, s, path) :: tl ->
        if (x, y) = target then Some (s, path)
        else
          let neighs =
            get_neighbours (x, y)
            |> List.filter_map (fun (x, y) ->
                   if
                     x >= 0 && x <= width && y >= 0 && y <= height
                     && (not (Hashtbl.mem obs (x, y)))
                     && not (Hashtbl.mem visited (x, y))
                   then (
                     Hashtbl.add visited (x, y) ();
                     Some (x, y, s + 1, Path.add (x, y) path))
                   else None)
          in

          aux (tl @ neighs)
  in

  aux [ (0, 0, 0, Path.singleton (0, 0)) ]

let () =
  let obstacles = Hashtbl.create 10 in

  let all_obstacles =
    Utils.read_lines "inputs/input_18.txt"
    |> List.filter_map (fun l ->
           match Utils.numbers_in_string l with
           | [ x; y ] -> Some (x, y)
           | _ -> None)
  in

  let rec insert_obstacles n rem_obs =
    match rem_obs with
    | [] -> rem_obs
    | _ when n = 0 -> rem_obs
    | hd :: tl ->
        Hashtbl.add obstacles hd ();
        insert_obstacles (n - 1) tl
  in

  let rem_obs = insert_obstacles bytes all_obstacles in

  let res1, path =
    path_len obstacles |> Option.value ~default:(-1, Path.empty)
  in

  let rec part2 path = function
    | [] -> None
    | hd :: tl ->
        Hashtbl.add obstacles hd ();
        if Path.mem hd path then
          match path_len obstacles with
          | Some (_, path) -> part2 path tl
          | None -> Some hd
        else part2 path tl
  in

  let x, y = part2 path rem_obs |> Option.value ~default:(-1, -1) in

  Printf.printf "Part1: %d\nPart2: %d,%d\n" res1 x y
