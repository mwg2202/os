#include <string>
class ErrorHandler{
    public:
        void proccessError(){
            formatError();
            printError();
        }
        void changeErrorLevel(int newLevel){
            errorLevel = newLevel;
        }

    private:
        std::string error;
        int errorLevel = 0;
        void formatError(){
            
        }
        
        int printError(){

        }
}