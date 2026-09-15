#include<iostream>
#include<vector>

int main(int argc,char* argv[]){
    std::vector<std::string> args;
    for(int x = 1;x < argc;x+=1){
        args.push_back(argv[x]);
    }

    if(args.size() > 0)
    std::cout << args[0]<<"\n";
	return 0;
}
