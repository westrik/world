//
//  MainWindowController.swift
//
//  Created by Matthew Westrik on 2020-04-02
//  Copyright © 2020 Matthew Westrik. All rights reserved.
//

import Cocoa

class MainWindowController: NSWindowController {

    static var shared: MainWindowController!

    override func windowDidLoad() {
        super.windowDidLoad()
        MainWindowController.shared = self
        setupView()
    }

    func setupView() {
        window?.recalculateKeyViewLoop()
    }
}
