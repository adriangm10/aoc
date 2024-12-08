module AntinodeSet = Set.Make (struct
  type t = int * int

  let compare = compare
end)

let calc_antinodes (y0, x0) (y1, x1) =
  let dy = y0 - y1 in
  let dx = x0 - x1 in

  let antinode1 = (y0 + dy, x0 + dx) in
  let antinode2 = (y1 - dy, x1 - dx) in

  (antinode1, antinode2)

let calc_antinodes_total (y0, x0) (y1, x1) map =
  let dy = y0 - y1 in
  let dx = x0 - x1 in

  let rec aux acc (dy, dx) = function
    | ay, _ when ay < 0 || ay >= List.length map -> acc
    | _, ax when ax < 0 || ax >= String.length (List.hd map) -> acc
    | ay, ax -> aux (AntinodeSet.add (ay, ax) acc) (dy, dx) (ay + dy, ax + dx)
  in

  AntinodeSet.union
    (aux AntinodeSet.empty (dy, dx) (y0, x0))
    (aux AntinodeSet.empty (-dy, -dx) (y1, x1))

let antinodes_of antennas =
  let rec aux acc last = function
    | [] -> acc
    | hd :: tl ->
        let ant1, ant2 = calc_antinodes last hd in
        let acc = AntinodeSet.add ant1 acc in
        let acc = AntinodeSet.add ant2 acc in
        AntinodeSet.union (aux acc hd tl) (aux acc last tl)
  in

  aux AntinodeSet.empty (List.hd antennas) (List.tl antennas)

let total_antinodes_of map antennas =
  let rec aux acc last = function
    | [] -> acc
    | hd :: tl ->
        let ants = calc_antinodes_total last hd map in
        let acc = AntinodeSet.union acc ants in
        AntinodeSet.union (aux acc hd tl) (aux acc last tl)
  in

  aux AntinodeSet.empty (List.hd antennas) (List.tl antennas)

let () =
  let antennas = Hashtbl.create 35 in
  let map = Utils.read_lines "inputs/input_8.txt" in
  map
  |> List.iteri (fun i l ->
         String.iteri
           (fun j c ->
             if c <> '.' then
               if Hashtbl.mem antennas c then
                 Hashtbl.replace antennas c ((i, j) :: Hashtbl.find antennas c)
               else Hashtbl.add antennas c [ (i, j) ])
           l);

  let res1 =
    Hashtbl.to_seq_values antennas
    |> Seq.map antinodes_of
    |> Seq.fold_left AntinodeSet.union AntinodeSet.empty
    |> AntinodeSet.filter (fun (x, y) ->
           x >= 0 && y >= 0
           && x < String.length (List.hd map)
           && y < List.length map)
    |> AntinodeSet.cardinal
  in

  let res2 =
    Hashtbl.to_seq_values antennas
    |> Seq.map (total_antinodes_of map)
    |> Seq.fold_left AntinodeSet.union AntinodeSet.empty
    |> AntinodeSet.cardinal
  in

  Printf.printf "part1: %d\npart2: %d\n" res1 res2
