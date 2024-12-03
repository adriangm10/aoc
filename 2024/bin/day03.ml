let find_muls input =
  let rec extract_mul s next_c acc =
    if s = "" then None
    else
      match (s.[0], next_c) with
      | 'm', "m" -> extract_mul (String.sub s 1 (String.length s - 1)) "u" ""
      | 'u', "u" -> extract_mul (String.sub s 1 (String.length s - 1)) "l" ""
      | 'l', "l" -> extract_mul (String.sub s 1 (String.length s - 1)) "(" ""
      | '(', "(" -> extract_mul (String.sub s 1 (String.length s - 1)) "d|," ""
      | ',', "d|," ->
          extract_mul (String.sub s 1 (String.length s - 1)) "d|)" (acc ^ ",")
      | ')', "d|)" -> Some acc
      | d, c when (c = "d|," || c = "d|)") && Utils.is_digit d ->
          extract_mul
            (String.sub s 1 (String.length s - 1))
            c
            (acc ^ String.make 1 d)
      | _, _ -> None
  in

  let rec is_do s next_c =
    if s = "" then false
    else
      match (s.[0], next_c) with
      | 'd', 'd' -> is_do (String.sub s 1 (String.length s - 1)) 'o'
      | 'o', 'o' -> is_do (String.sub s 1 (String.length s - 1)) '('
      | '(', '(' -> is_do (String.sub s 1 (String.length s - 1)) ')'
      | ')', ')' -> true
      | _, _ -> false
  in

  let rec is_dont s next_c =
    if s = "" then false
    else
      match (s.[0], next_c) with
      | 'd', 'd' -> is_dont (String.sub s 1 (String.length s - 1)) 'o'
      | 'o', 'o' -> is_dont (String.sub s 1 (String.length s - 1)) 'n'
      | 'n', 'n' -> is_dont (String.sub s 1 (String.length s - 1)) '\''
      | '\'', '\'' -> is_dont (String.sub s 1 (String.length s - 1)) 't'
      | 't', 't' -> is_dont (String.sub s 1 (String.length s - 1)) '('
      | '(', '(' -> is_dont (String.sub s 1 (String.length s - 1)) ')'
      | ')', ')' -> true
      | _, _ -> false
  in

  let rec extract_mul_list s _do acc =
    if s = "" then acc
    else if is_do s 'd' then
      extract_mul_list
        (* 4 is the number of chars in do() *)
        (String.sub s 4 (String.length s - 4))
        true acc
    else if is_dont s 'd' then
      extract_mul_list
        (* 7 is the number of chars in don't() *)
        (String.sub s 7 (String.length s - 7))
        false acc
    else if _do then
      match extract_mul s "m" "" with
      | Some ns ->
          let nums = String.split_on_char ',' ns in
          let num1 = List.nth nums 0 in
          let num2 = List.nth nums 1 in
          if String.length num1 <= 3 && String.length num2 <= 3 then
            extract_mul_list
              (* 5 is the number of chars in mul() *)
              (String.sub s
                 (String.length ns + 5)
                 (String.length s - String.length ns - 5))
              _do
              ((int_of_string num1, int_of_string num2) :: acc)
          else
            extract_mul_list
              (String.sub s
                 (String.length ns + 5)
                 (String.length s - String.length ns - 5))
              _do acc
      | None -> extract_mul_list (String.sub s 1 (String.length s - 1)) _do acc
    else extract_mul_list (String.sub s 1 (String.length s - 1)) _do acc
  in

  List.rev (extract_mul_list input true [])

let () =
  let muls =
    Utils.read_all "inputs/input_3.txt"
    |> find_muls
    |> List.map (fun (x, y) -> x * y)
    |> List.fold_left ( + ) 0
  in

  print_endline (string_of_int muls)
