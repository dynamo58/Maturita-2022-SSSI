from sys import argv as args

try:
	with open(f"{args[-1]}", "r") as f:
		with open(f"{args[-1][:-4]}_nbsp{args[-1][-4:]}", "w") as g:
			out = ""

			is_minted_env = False
			is_document_env = False

			for line in f.readlines():
				append = line
				
				if "begin{document}" in line:
					is_document_env = True
				
				if "{minted}" in line:
					is_minted_env != is_minted_env

				if is_document_env and not is_minted_env:
					for char in ["a", "v", "s", "u", "z", "k", "i"]:
						append = append.replace(" " + char + " ", " " + char + "~")
						append = append.replace(" " + char.upper() + " ", " " + char.upper() + "~")

						append = append.replace("." + char + " ", " " + char + "~")
						append = append.replace("." + char.upper() + " ", " " + char.upper() + "~")
				
				out += append
			
			g.write(out)

except Exception as e:
	print(e)