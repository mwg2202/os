#include "./ErrorHandler.cpp"


class GraphicsController{
    public:
        GraphicsController(ErrorHandler& errorHandler){
            setVideoMode(3);
        }
        int setVideoMode(int newVideoMode){
            switch(newVideoMode) {
                case 0: /* text 40*25 16 color (mono) */
                    std::string error = "VGA/VESA Mode Not Supported"
                    errorHandler.proccessError(error);
                    return 1;
                    break;
                case 1: /* text 40*25 16 color */
                    return 1;
                    break;
                case 2: /* text 80*25 16 color (mono) */
                    return 1;
                    break;
                case 3: /* text 80*25 16 color */
                    videoMode = newVideoMode;
                    return 0;
                    break;
                case 4: /* CGA 320*200 4 color */
                    return 1;
                    break;
                case 5: /* CGA 320*200 4 color (mono) */
                    return 1;
                    break;
                case 6: /* CGA 640*200 2 color */
                    return 1;
                    break;
                case 7: /* MDA monochrome text 80*25 */
                    return 1;
                    break;
                case 8:
                    return 1;
                    break;
                case 9:
                    return 1;
                    break;
                case 10:
                    return 1;
                    break;
                case 13: /* VGA 320*200 16 Color */
                    return 1;
                    break;
                case 14: /* VGA 640*200 16 Color */
                    return 1;
                    break;
                case 15: /* VGA 640*350 mono */
                    return 1;
                    break;
                case 16: /* VGA 640*350 16 Color */
                    return 1;
                    break;
                case 17: /* VGA 640*480 mono */
                    return 1;
                    break;
                case 18: /* VGA 640*480 16 Color */
                    return 1;
                    break;
                case 19: /* VGA 320*200 256 Color */
                    return 1;
                    break;
                default:

                    return 1;
            }

            return 0;
        }
        void printText(std::string text){
            switch(videoMode){
                case 0:
                case 1:
                case 2:
                case 3:
                default:
                std::string error = "Video Mode Doesn't Support Text";
                errorHandler.processError(error);
            }
        }
    
    private:
        ErrorHandler errorHandler();
        int videoMode;
        void sendError(std::string error){
            errorHandler.processError(error);
        }
}