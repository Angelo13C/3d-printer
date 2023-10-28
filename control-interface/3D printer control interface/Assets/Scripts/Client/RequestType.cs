using System;

namespace Client
{
    public enum RequestType
    {
        ListFiles,
        DeleteFile,
        PrintFile,
        SendFile,
        GetPrintStatus,
        PauseOrResume,
        PrinterState,
    }
    
    public static class RequestTypeExtensions
    {
        public static string ToUri(this RequestType requestType)
        {
            switch(requestType)
            {
                case RequestType.ListFiles:     return "list-files";
                case RequestType.DeleteFile:    return "delete-file";
                case RequestType.PrintFile:     return "print-file";
                case RequestType.SendFile:      return "send-file";
                case RequestType.GetPrintStatus: return "print-status";
                case RequestType.PauseOrResume: return "pause-or-resume";
                case RequestType.PrinterState: return "printer-state";
                default: throw new NotImplementedException();
            }
        }
    }
}