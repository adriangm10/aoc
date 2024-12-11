let blink stones n =
  let mem = Hashtbl.create n in

  let rec aux stone n =
    match Hashtbl.find_opt mem (stone, n) with
    | Some v -> v
    | None ->
        let s = string_of_int stone in
        let len = String.length s in
        let v =
          if n = 1 then if len mod 2 = 0 then 2 else 1
          else if stone = 0 then aux 1 (n - 1)
          else if len mod 2 = 0 then
            let rock1 = String.sub s 0 (len / 2) |> int_of_string in
            let rock2 = String.sub s (len / 2) (len / 2) |> int_of_string in
            aux rock1 (n - 1) + aux rock2 (n - 1)
          else aux (stone * 2024) (n - 1)
        in
        Hashtbl.add mem (stone, n) v;
        v
  in

  List.fold_left (fun acc stone -> acc + aux stone n) 0 stones

let () =
  let rocks =
    Utils.read_all "inputs/input_11.txt"
    |> Utils.split_on ' ' |> List.map int_of_string
  in

  let res1 = blink rocks 25 in
  let res2 = blink rocks 100 in
  Printf.printf "Part1: %d\nPart2: %d\n" res1 res2
