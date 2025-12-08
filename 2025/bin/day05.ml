module RangeSet = Set.Make (struct
  type t = int * int

  let compare = compare
end)

let parse_range s =
  match String.split_on_char '-' s with
  | [ x1; x2 ] -> (int_of_string x1, int_of_string x2)
  | _ -> raise (Failure "couldn't parse range")

let check_id ranges id =
  List.exists (fun (x1, x2) -> x1 <= id && id <= x2) ranges

let fresh_ids ranges =
  let insert_range (x1, x2) ranges =
    match
      RangeSet.find_first_opt
        (fun (r1, r2) ->
          (r1 <= x1 && x1 <= r2 && x2 > r2) (* r1 <= x1 <= r2 and x2 > r2 *)
          || (r1 <= x2 && x2 <= r2 && x1 < r1) (* r1 <= x2 <= r2 and x1 < r1 *)
          || (r1 <= x2 && x2 <= r2 && r1 <= x1 && x1 <= r2))
          (* (x1, x2) inside (r1, r2) *)
        ranges
    with
    | Some (r1, r2) ->
        let ranges = RangeSet.remove (r1, r2) ranges in
        if r1 <= x1 && x1 <= r2 && x2 > r2 then RangeSet.add (r1, x2) ranges
        else if r1 <= x2 && x2 <= r2 && x1 < r1 then
          RangeSet.add (x1, r2) ranges
        else ranges
    | None -> RangeSet.add (x1, x2) ranges
  in

  let rec unite_ranges ranges = function
    | [] -> ranges
    | (x1, x2) :: t -> unite_ranges (insert_range (x1, x2) ranges) t
  in

  let curr_ranges = ref (unite_ranges RangeSet.empty ranges) in
  for _ = 1 to 20 do
    curr_ranges := unite_ranges RangeSet.empty (RangeSet.to_list !curr_ranges)
  done;
  !curr_ranges

let () =
  let ranges, ids =
    match
      Utils.split_string ~pattern:"\n\n" (Utils.read_file "inputs/05.txt")
    with
    | [ ranges; ids ] ->
        ( String.split_on_char '\n' ranges |> List.map parse_range,
          String.split_on_char '\n' ids
          |> List.filter_map (fun x ->
              try Some (int_of_string x) with _ -> None) )
    | _ -> raise Exit
  in

  let res1 = List.filter (check_id ranges) ids in
  let res2 = fresh_ids ranges in
  RangeSet.iter (fun (x1, x2) -> Printf.printf "(%d, %d),\n" x1 x2) res2;

  Printf.printf "Part1: %d\nPart2: %d\n" (List.length res1)
    (RangeSet.fold (fun (x1, x2) a -> a + (x2 - x1 + 1)) res2 0)
