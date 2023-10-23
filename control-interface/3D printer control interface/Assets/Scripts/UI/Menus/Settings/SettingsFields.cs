using UnityEngine;

namespace UI.Menus.Settings
{
    [CreateAssetMenu(fileName = "Settings fields", menuName = "Scriptable Objects/Settings fields")]
    public class SettingsFields : ScriptableObject
    {
        [field: SerializeField] public Block[] Blocks { get; private set;  }
        
        [System.Serializable]
        public struct Block
        {
            public string Title;
            public Field[] Fields;

            [System.Serializable]
            public struct Field
            {
                public string Name;
                public FieldType Type;

                public enum FieldType
                {
                    Int,
                    Distance
                }
            }
        }
    }
}