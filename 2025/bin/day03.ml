let best2 bank =
  let bank =
    List.init (String.length bank) (String.get bank)
    |> List.map (fun c -> int_of_char c - int_of_char '0')
  in

  let rec aux max1 max2 first1 = function
    | [] -> (max1, max2, first1)
    | h :: t ->
        if h > max1 then aux h (if List.length t > 0 then 0 else max1) false t
        else if h > max2 then aux max1 h true t
        else aux max1 max2 first1 t
  in

  aux 0 0 false bank

let () =
  let sol =
    Utils.read_lines "inputs/03.txt"
    |> List.fold_left
         (fun a bank ->
           let m1, m2, first1 = best2 bank in
           if first1 then ((m1 * 10) + m2) :: a else ((m2 * 10) + m1) :: a)
         []
    |> List.fold_left ( + ) 0
  in

  print_int sol;
  print_newline ()
