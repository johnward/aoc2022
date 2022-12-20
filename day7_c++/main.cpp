#include <iostream>
#include <string>
#include <unordered_map>
#include <fstream>

int main() {
  // Use a map to store the total sizes of each directory
  std::unordered_map<std::string, int> dir_sizes;


  // Read from the text file
  std::ifstream MyReadFile("input.txt");

  // Set the initial current directory to the root directory "/"
  std::string current_dir = "/";
  std::string myText;

  // Read the input data line by line
  std::string line;
  while (std::getline (MyReadFile, myText)) {
  //while (std::getline(std::cin, line)) {
    // If the line starts with "$ cd", change the current directory

    printf("%s\n",myText.c_str());

    if (line.substr(0, 4) == "$ cd") {
      std::string dir = line.substr(5);
      if (dir == "/") {
        // If the directory is "/", switch to the root directory
        current_dir = "/";
      } else if (dir.substr(0, 2) == "..") {
        // If the directory starts with "..", move up one level
        current_dir = current_dir.substr(0, current_dir.find_last_of("/"));
      } else {
        // Otherwise, move down one level into the specified directory
        current_dir += ("/" + dir);
      }
    } else if (line.substr(0, 3) == "$ ls") {
      // If the line starts with "$ ls", parse the list of files and directories
      std::string data = line.substr(4);
      int pos = 0;
      while (pos < data.size()) {
        // Parse each file or directory in the list
        int space_pos = data.find(" ", pos);
        std::string item = data.substr(pos, space_pos - pos);
        pos = space_pos + 1;

        // Check if the item is a file or a directory
        if (item[item.size() - 1] == '.') {
          // If it's a file, add its size to the current directory's total size
          int size = std::stoi(item.substr(0, item.find(" ")));
          dir_sizes[current_dir] += size;
        } else {
          // If it's a directory, add it to the map with a size of 0
          dir_sizes[current_dir + "/" + item] = 0;
        }
      }
    }
  }

  // Calculate the sum of the total sizes of all directories with a size of at most 100000
  int total_size = 0;
  for (const auto& [dir, size] : dir_sizes) {
    if (size <= 100000) {
      total_size += size;
    }
  }

  // Print the result
  std::cout << "Total size: " << total_size << std::endl;

  return 0;
}

