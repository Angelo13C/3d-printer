using TMPro;
using Unity.VectorGraphics;
using UnityEngine;
using UnityEngine.UI;

namespace UI.Menus.Base
{
    public class MenuUI : MonoBehaviour
    {
        [Header("Colors")]
        [SerializeField] private Color _selectedMenuButtonColor = Color.grey;
        [SerializeField] private Color _unselectedMenuButtonColor = Color.white;
        
        [Header("Selection image")]
        [SerializeField] private RectTransform _menuSelection;
        [SerializeField] private int _menuSelectionWidthOffset = 20;
        
        [Space]
        [SerializeField] private uint _menuOpenedOnStartup;
        
        [System.Serializable]
        private struct Menu
        {
            public Canvas Canvas;
            public Button MenuButton;
        }
        [SerializeField] private Menu[] _menus;

        private Menu? _currentlySelectedMenu = null;
        
        private void Awake()
        {
            foreach (var menu in _menus)
                menu.Canvas.gameObject.SetActive(false);
            
            if(_menuOpenedOnStartup < _menus.Length)
                OpenMenu(_menus[_menuOpenedOnStartup].Canvas);
        }

        public void OpenMenu(Canvas menu)
        {
            if (_currentlySelectedMenu != null)
            {
                _currentlySelectedMenu.Value.Canvas.gameObject.SetActive(false);
                _currentlySelectedMenu.Value.MenuButton.GetComponentInChildren<TextMeshProUGUI>().color = _unselectedMenuButtonColor;
                _currentlySelectedMenu.Value.MenuButton.GetComponentInChildren<SVGImage>().color = _unselectedMenuButtonColor;
            }

            foreach (var possibleMenu in _menus)
            {
                if (possibleMenu.Canvas == menu)
                    _currentlySelectedMenu = possibleMenu;
            }

            if (_currentlySelectedMenu != null)
            {
                _currentlySelectedMenu.Value.Canvas.gameObject.SetActive(true);
                _currentlySelectedMenu.Value.MenuButton.GetComponentInChildren<TextMeshProUGUI>().color = _selectedMenuButtonColor;
                _currentlySelectedMenu.Value.MenuButton.GetComponentInChildren<SVGImage>().color = _selectedMenuButtonColor;
                
                _menuSelection.gameObject.SetActive(true);
                var menuSelectionTransform = _currentlySelectedMenu.Value.MenuButton.GetComponent<RectTransform>();
                _menuSelection.position = menuSelectionTransform.position;
                var size = _menuSelection.sizeDelta;
                size.x = menuSelectionTransform.sizeDelta.x + _menuSelectionWidthOffset;
                _menuSelection.sizeDelta = size;
            }
        }

#if UNITY_EDITOR
        private void OnValidate()
        {
            _menuOpenedOnStartup = (uint) Mathf.Clamp(_menuOpenedOnStartup, 0, _menus.Length);
        }
#endif
    }
}