//
//  ASApp.swift
//
//  Created by Matthew Westrik on 2020-04-02
//  Copyright © 2020 Matthew Westrik. All rights reserved.
//

import Cocoa

// This subclass stands for "AppleScriptableApplication".
// It's a simple entry point to provide basic AppleScript
// capabilities to your app. Take a look at macOS/Resources/westrikworld.sdef
class ASApplication: NSApplication {

    // This is just an example command to see how it relates to the
    // corresponding definition in macOS/Resources/westrikworld.sdef
    func openMainWindow() {
        if let delegate = delegate as? AppDelegate {
            delegate.showMainWindow(nil)
        }
    }
}
