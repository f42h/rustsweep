require "http/client"
require "uri"


# Generate OUI lookup source: `crystal run oui_ieee_dbdl.cr`

ieee_sources = [
    "https://standards-oui.ieee.org/oui/oui.csv",
    "https://standards-oui.ieee.org/oui36/oui36.csv",
    "https://standards-oui.ieee.org/oui28/mam.csv",
    "https://standards-oui.ieee.org/iab/iab.csv"
]

def get_csv_basename(url)
    parsed_url = URI.parse(url)
    path = parsed_url.path

    File.basename(path)
end

# Output location
output_directory = "ieee_dbs_csv/"

if Dir.exists?(output_directory)
    Dir.each_child(output_directory) do |file|
        file_path = File.join(output_directory, file)
        File.delete(file_path)
    end
else
    Dir.mkdir(output_directory)
end

# Download CSVs
ieee_sources.each do |url|
    csv_basename = get_csv_basename(url)

    puts "Starting download: #{url} -> #{csv_basename}"

    output_csv = "#{output_directory}#{csv_basename}"

    HTTP::Client.get(url) do |response|
      File.write(output_csv, response.body_io)
    end

    puts "Download complete: #{output_csv}"
end

output_txt = "../oui.txt"

# Check if the file exists and remove it
if File.exists?(output_txt)
    File.delete(output_txt)
end

def output_txt_append(file, content)
    File.open(file, "a+") do |file|
        file.print content
    end
end

Dir.each_child(output_directory) do |file|
    csv_path = File.join(output_directory, file)

    line_counter = 0

    File.each_line(csv_path) do |line|
        # Skip first line
        line_counter += 1
        next if line_counter == 1

        tok = line.split(",")
        assignment = tok[1]  
        organisation = tok[2] 

        content = "#{assignment},#{organisation}\n"
        
        output_txt_append(output_txt, content)
    end
end

puts "Created: #{output_txt}"