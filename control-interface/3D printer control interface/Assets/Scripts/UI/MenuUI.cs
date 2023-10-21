using System;
using UnityEngine;
using UnityEngine.UI;

namespace UI
{
    public class MenuUI : MonoBehaviour
    {
        [SerializeField] private Canvas[] _menus;

        private Canvas _currentlySelectedMenu = null;
        
        private void Awake()
        {
            foreach (var menu in _menus)
                menu.enabled = false;
        }

        public void OpenMenu(Canvas menu)
        {
            if (_currentlySelectedMenu != null)
                _currentlySelectedMenu.enabled = false;

            _currentlySelectedMenu = menu;
            _currentlySelectedMenu.enabled = true;
        }
    }
}