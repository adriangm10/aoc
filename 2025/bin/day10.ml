let parse_lights diag =
  let lights = Array.make (String.length diag - 2) 0 in
  let diag = String.sub diag 1 (String.length diag - 2) in

  diag
  |> String.iteri (fun i c ->
      match c with
      | '[' | ']' | '.' -> ()
      | '#' -> lights.(i) <- 1
      | _ -> raise (Failure "parse_lights"));
  lights

let parse_buttons_or_joltage diag =
  String.sub diag 1 (String.length diag - 2)
  |> String.trim |> String.split_on_char ',' |> List.map int_of_string
  |> Array.of_list


let find_min_toggles machine =
  let push_button lights button =
    Array.iter (fun x -> lights.(x) <- Int.abs (lights.(x) - 1)) button
  in

  (* joltage, buttons, lights *)
  let buttons = List.tl (List.rev (List.tl machine)) in
  let target_lights = List.rev machine |> List.hd in
  let init_lights = Array.make (Array.length target_lights) 0 in

  let rec aux visited = function
    | [] -> raise (Failure "find_min_toggles")
    | (lights, n) :: tl ->
        if Array.for_all2 (fun x y -> x == y) lights target_lights then n
        else if List.mem lights visited then aux visited tl
        else
          let neighbours =
            buttons
            |> List.map (fun b ->
                let ls = Array.copy lights in
                push_button ls b;
                (ls, n + 1))
          in
          aux (visited @ [ lights ]) (tl @ neighbours)
  in

  aux [] [ (init_lights, 0) ]

let find_min_presses machine =
  let push_button lights button =
    Array.iter (fun x -> lights.(x) <- lights.(x) + 1) button
  in

  (* joltage, buttons, lights *)
  let buttons = List.tl (List.rev (List.tl machine)) in
  let target_joltage = List.hd machine in
  let init_joltage = Array.make (Array.length target_joltage) 0 in

  let rec aux visited = function
    | [] -> raise (Failure "find_min_presses")
    | (joltage, n) :: tl ->
        if Array.for_all2 (fun x y -> x == y) joltage target_joltage then n
        else if Array.exists2 (fun x y -> x > y) joltage target_joltage then
          aux visited tl
        else if List.mem joltage visited then aux visited tl
        else
          let neighbours =
            buttons
            |> List.map (fun b ->
                let ls = Array.copy joltage in
                push_button ls b;
                (ls, n + 1))
          in
          aux (visited @ [ joltage ]) (tl @ neighbours)
  in

  aux [] [ (init_joltage, 0) ]

let () =
  (* joltage, buttons, lights *)
  let machines =
    Utils.read_lines "inputs/10.txt"
    |> List.map (fun l ->
        String.split_on_char ' ' l
        |> List.fold_left
             (fun acc x ->
               match String.get x 0 with
               | '[' -> parse_lights x :: acc
               | '(' | '{' -> parse_buttons_or_joltage x :: acc
               | _ -> raise (Failure "parse input"))
             [])
  in

  let res1 =
    machines
    |> List.fold_left (fun acc machine -> acc + find_min_toggles machine) 0
  in

  let res2 = 
    machines
    |> List.fold_left (fun acc machine -> acc + find_min_presses machine) 0
  in

  Printf.printf "Part1: %d\nPart2: %d\n" res1 res2
