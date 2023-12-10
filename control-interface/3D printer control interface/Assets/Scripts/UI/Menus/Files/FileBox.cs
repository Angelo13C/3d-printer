using System;
using TMPro;
using UnityEngine;

namespace UI.Menus.Files
{
    public class FileBox : MonoBehaviour
    {
        [field: SerializeField] public TextMeshProUGUI FileName { get; private set; }
        [field: SerializeField] public TextMeshProUGUI FileSize { get; private set; }
        
        public FileId FileId { get; set; }

        public Action<FileId> OnPrintButtonPressed;
        public Action<FileId> OnDeleteButtonPressed;

        public void PressPrintButton() => OnPrintButtonPressed.Invoke(FileId);
        public void PressDeleteButton() => OnDeleteButtonPressed.Invoke(FileId);
    }

    [System.Serializable]
    public struct FileId
    {
        public uint FileID;
    }
}