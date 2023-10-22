using System.IO;
using System.Net.Http;
using Client;
using SFB;
using UnityEngine;

namespace UI.Menus.Files
{
    public class SendFileToPrinter : MonoBehaviour
    {
        [SerializeField] private HttpsClient _httpsClient;
    
        public void OpenFilePicker()
        {
            if (_httpsClient == null)
                return;
        
            var extensions = new [] {
                new ExtensionFilter("G-code File ", "gcode"),
            };
            StandaloneFileBrowser.OpenFilePanelAsync("Open File", "", extensions, false, async paths =>
            {
                foreach (var path in paths)
                {
                    if (File.Exists(path))
                    {
                        var file = File.OpenRead(path);
                        var content = new StreamContent(file);
                        var request = new HttpRequestMessage
                        {
                            Method = HttpMethod.Post,
                            Content = content,
                            Headers =
                            {
                                { "length", file.Length.ToString() },
                                { "name", Path.GetFileNameWithoutExtension(file.Name) }
                            }
                        };
                        await _httpsClient.SendRequest(request, RequestType.SendFile);
                    }
                }
            });
        }
    }
}