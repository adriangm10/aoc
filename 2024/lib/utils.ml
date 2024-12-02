let read_lines file_name =
  let ic = open_in file_name in
  let try_line () = try Some (input_line ic) with End_of_file -> None in

  let rec loop acc =
    match try_line () with
    | Some line -> loop (line :: acc)
    | None ->
        close_in ic;
        List.rev acc
  in

  loop []

let split_on c s =
   String.split_on_char c s |> List.filter (fun s -> s <> "")
