using System.Collections.Generic;
using System.Net.Http;
using System.Text;
using Client;
using UnityEngine;
using UnityEngine.UI;

namespace UI.Menus.Files
{
    [RequireComponent(typeof(VerticalLayoutGroup))]
    public class FileList : MonoBehaviour
    {
        [SerializeField] private HttpsClient _httpsClient;
        [SerializeField] private GameObject _filePrefab;

        [SerializeField] private GameObject _emptyFileListMessage;

        [SerializeField] private float _secondsBetweenRefreshes = 5;

        private List<HttpResponse.File> _currentlyStoredFiles = new(20);

        [System.Serializable]
        struct HttpResponse
        {
            public List<File> Files;

            [System.Serializable]
            public struct File
            {
                public string Name;
                public ulong SizeInBytes;
                public FileId ID;
            }
        }

        private void Awake() => InvokeRepeating(nameof(Refresh), 0, _secondsBetweenRefreshes);

        private async void Refresh()
        {
            _emptyFileListMessage.SetActive(_currentlyStoredFiles.Count == 0);
            
            if (!gameObject.activeInHierarchy)
                return;
            
            var request = new HttpRequestMessage
            {
                Method = HttpMethod.Get
            };
            var response = await _httpsClient.SendRequest(request, RequestType.ListFiles);
            if (response == null || response.Content == null)
                return;
            
            var responseBodyString = await response.Content.ReadAsStringAsync();
            var responseBody = JsonUtility.FromJson<HttpResponse>(responseBodyString);
            _currentlyStoredFiles = responseBody.Files;
            
            _emptyFileListMessage.SetActive(_currentlyStoredFiles.Count == 0);
        }

        private void OnEnable() => Refresh();

        private void Update()
        {
            for (var i = 0; i < transform.childCount - _currentlyStoredFiles.Count; i++)
                Destroy(transform.GetChild(i).gameObject);
            for (var i = 0; i < _currentlyStoredFiles.Count - transform.childCount; i++) 
                Instantiate(_filePrefab, transform);

            for (var i = 0; i < _currentlyStoredFiles.Count; i++)
            {
                var childFileBox = transform.GetChild(i).GetComponent<FileBox>();
                childFileBox.FileName.text = _currentlyStoredFiles[i].Name;
                childFileBox.FileSize.text = "Size: " + BytesToString(_currentlyStoredFiles[i].SizeInBytes);
                childFileBox.FileId = _currentlyStoredFiles[i].ID;
            }
        }
        
        private static string BytesToString(ulong byteCount)
        {
            string[] suffixes = { "B", "KB", "MB" };
            const ulong OrderSize = 1024;
            
            var order = 0;
            var byteCountFloat = (float) byteCount;
            while (byteCountFloat >= OrderSize && order < suffixes.Length - 1) {
                order++;
                byteCountFloat /= OrderSize;
            }
            
            return string.Format("{0:0.##}{1}", byteCountFloat, suffixes[order]);;
        }
    }
}