let next_num x =
  let x = ((x * 64) lxor x) mod 16777216 in
  let x = ((Float.floor (float_of_int x /. float_of_int 32) |> int_of_float) lxor x) mod 16777216 in
  ((x * 2048) lxor x) mod 16777216

let rec next_n_num n x =
  if n = 0 then x
  else next_n_num (n - 1) (next_num x)

let () =
  let nums =
    Utils.read_lines "inputs/input_22.txt" |> List.map int_of_string
  in

  let nums_2000 = List.map (next_n_num 2000) nums in

  let res1 = List.fold_left ( + ) 0 nums_2000 in
  (* List.iter2 (fun x y -> Printf.printf "%d: %d\n" x y) nums nums_2000; *)

  Printf.printf "Part1: %d\n" res1
